//! Core of Bron-Kerbosch algorithms using degeneracy ordering and multiple threads.

pub use super::bron_kerbosch_pivot::PivotChoice;
use super::bron_kerbosch_pivot::visit;
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::graph_degeneracy::DegeneracyOrder;
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
        let (visit_tx, visit_rx) = crossbeam_channel::bounded(64);
        scope.spawn(move |_| dispatch(graph, visit_tx));
        for _ in 0..num_visiting_threads {
            let thread_visit_rx = visit_rx.clone();
            let thread_consumer = consumer.clone();
            scope.spawn(move |_| descend(graph, thread_consumer, pivot_selection, thread_visit_rx));
        }
    })
    .unwrap();
}

struct VisitJob<VertexSet> {
    start: Vertex,
    candidates: VertexSet,
    excluded: VertexSet,
}

fn dispatch<VertexSet>(graph: &Graph<VertexSet>, visit_tx: Sender<VisitJob<VertexSet>>)
where
    VertexSet: VertexSetLike,
{
    DegeneracyOrder::on(graph).apply(|v, attorney| {
        let (neighbouring_candidates, neighbouring_excluded) = attorney.partition_neighbours(v);
        let visit = VisitJob {
            start: v,
            candidates: neighbouring_candidates,
            excluded: neighbouring_excluded,
        };
        visit_tx.send(visit).unwrap();
    })
}

fn descend<VertexSet>(
    graph: &Graph<VertexSet>,
    mut consumer: CliqueConsumer,
    pivot_selection: PivotChoice,
    visit_rx: Receiver<VisitJob<VertexSet>>,
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
