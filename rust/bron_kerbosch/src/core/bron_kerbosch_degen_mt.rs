//! Core of Bron-Kerbosch algorithms using degeneracy ordering and multiple threads.

pub use super::bron_kerbosch_pivot::PivotChoice;
use super::bron_kerbosch_pivot::visit;
use super::clique::CliqueConsumer;
use super::graph::Graph;
use super::graph_degeneracy::degeneracy_iter;
use super::pile::Pile;
use super::vertex::Vertex;
use super::vertexsetlike::VertexSetLike;
use crossbeam_channel::{Receiver, Sender};

pub fn explore_with_pivot_multithreaded<VertexSet>(
    graph: &Graph<VertexSet>,
    consumer: CliqueConsumer,
    pivot_selection: PivotChoice,
    num_visiting_threads: usize,
) where
    VertexSet: VertexSetLike,
{
    crossbeam::thread::scope(|scope| {
        let (start_tx, start_rx) = crossbeam_channel::bounded(64);
        let (visit_tx, visit_rx) = crossbeam_channel::bounded(64);

        scope.spawn(move |_| initiate(graph, start_tx));
        scope.spawn(move |_| dispatch(graph, start_rx, visit_tx));

        for _ in 0..num_visiting_threads {
            let thread_visit_rx = visit_rx.clone();
            let thread_consumer = consumer.clone();
            scope.spawn(move |_| descend(graph, pivot_selection, thread_visit_rx, thread_consumer));
        }
        drop(visit_rx);
    })
    .unwrap();
}

struct VisitJob<VertexSet> {
    start: Vertex,
    candidates: VertexSet,
    excluded: VertexSet,
}

fn initiate<VertexSet>(graph: &Graph<VertexSet>, start_tx: Sender<(Vertex, VertexSet)>)
where
    VertexSet: VertexSetLike,
{
    for pair in degeneracy_iter(graph) {
        start_tx.send(pair).unwrap();
    }
}

fn dispatch<VertexSet>(
    graph: &Graph<VertexSet>,
    start_rx: Receiver<(Vertex, VertexSet)>,
    visit_tx: Sender<VisitJob<VertexSet>>,
) where
    VertexSet: VertexSetLike,
{
    // In this initial iteration, we don't need to represent the set of candidates
    // because all neighbours are candidates until excluded.
    while let Ok((v, neighbouring_excluded)) = start_rx.recv() {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        debug_assert!(neighbouring_excluded.len() < neighbours.len());
        let neighbouring_candidates = neighbours.difference_collect(&neighbouring_excluded);
        let visit = VisitJob {
            start: v,
            candidates: neighbouring_candidates,
            excluded: neighbouring_excluded,
        };
        visit_tx.send(visit).unwrap();
    }
}

fn descend<VertexSet>(
    graph: &Graph<VertexSet>,
    pivot_selection: PivotChoice,
    visit_rx: Receiver<VisitJob<VertexSet>>,
    mut consumer: CliqueConsumer,
) where
    VertexSet: VertexSetLike,
{
    while let Ok(job) = visit_rx.recv() {
        visit(
            graph,
            &mut consumer,
            pivot_selection,
            job.candidates,
            job.excluded,
            &Pile::from(job.start),
        );
    }
}
