//! Core of Bron-Kerbosch algorithms using degeneracy ordering and multiple threads.

pub use super::bron_kerbosch_pivot::PivotChoice;
use super::bron_kerbosch_pivot::visit;
use super::clique_consumer::CliqueConsumer;
use super::degeneracy::Degeneracy;
use super::graph::Graph;
use super::pile::Pile;
use super::vertex::Vertex;
use super::vertexsetlike::VertexSetLike;
use crossbeam_channel::{Receiver, Sender};

pub fn explore_with_degeneracy_mt<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    consumer: Consumer,
    pivot_selection: PivotChoice,
    num_visiting_threads: usize,
) -> Consumer::Harvest
where
    VertexSet: VertexSetLike + Sync,
    Consumer: CliqueConsumer + Clone + Send,
{
    let mut thread_consumers = vec![consumer; num_visiting_threads];
    crossbeam::thread::scope(|scope| {
        let (visit_tx, visit_rx) = crossbeam_channel::bounded(64);
        scope.spawn(move |_| dispatch(graph, visit_tx));
        for thread_consumer in &mut thread_consumers {
            let thread_visit_rx = visit_rx.clone();
            scope.spawn(|_| descend(graph, thread_consumer, pivot_selection, thread_visit_rx));
        }
    })
    .unwrap();

    let mut it = thread_consumers.into_iter();
    let first = it.next().unwrap().harvest();
    it.fold(first, |sum: Consumer::Harvest, tid| {
        Consumer::combine(sum, tid.harvest())
    })
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
    Degeneracy::on(graph).apply(|v, attorney| {
        let (neighbouring_candidates, neighbouring_excluded) = attorney.partition_neighbours(v);
        let visit = VisitJob {
            start: v,
            candidates: neighbouring_candidates,
            excluded: neighbouring_excluded,
        };
        visit_tx.send(visit).unwrap();
    })
}

fn descend<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    consumer: &mut Consumer,
    pivot_selection: PivotChoice,
    visit_rx: Receiver<VisitJob<VertexSet>>,
) where
    VertexSet: VertexSetLike,
    Consumer: CliqueConsumer,
{
    while let Ok(job) = visit_rx.recv() {
        visit(
            graph,
            consumer,
            pivot_selection,
            job.candidates,
            job.excluded,
            &Pile::from(job.start),
        );
    }
}
