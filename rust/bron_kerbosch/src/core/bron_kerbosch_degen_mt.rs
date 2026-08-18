//! Core of Bron-Kerbosch algorithms using degeneracy ordering and multiple threads.

use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::clique_consumer::CliqueConsumer;
use super::degeneracy::Degeneracy;
use super::pile::Pile;
use crate::{CliqueAccumulator, Graph, Vertex, VertexSetLike};
use crossbeam_channel::{Receiver, Sender};

pub fn explore_with_degeneracy_mt<VertexSet, Accumulator>(
    graph: &Graph<VertexSet>,
    min_clique_size: usize,
    accumulator: Accumulator,
    pivot_selection: PivotChoice,
    num_visiting_threads: usize,
) -> Accumulator::Harvest
where
    VertexSet: VertexSetLike + Sync,
    Accumulator: CliqueAccumulator + Clone + Send,
{
    let mut thread_accumulators = vec![accumulator; num_visiting_threads];
    crossbeam::thread::scope(|scope| {
        let (visit_tx, visit_rx) = crossbeam_channel::bounded(64);
        scope.spawn(move |_| dispatch(graph, visit_tx));
        for accu in &mut thread_accumulators {
            let thread_visit_rx = visit_rx.clone();
            let thread_consumer = CliqueConsumer {
                min_clique_size,
                accu,
            };
            scope.spawn(|_| descend(graph, thread_consumer, pivot_selection, thread_visit_rx));
        }
    })
    .unwrap();

    let mut it = thread_accumulators.into_iter();
    let mut first = it.next().unwrap();
    it.for_each(|next| first.absorb(next));
    first.harvest()
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

fn descend<VertexSet, Accumulator>(
    graph: &Graph<VertexSet>,
    mut consumer: CliqueConsumer<Accumulator>,
    pivot_selection: PivotChoice,
    visit_rx: Receiver<VisitJob<VertexSet>>,
) where
    VertexSet: VertexSetLike,
    Accumulator: CliqueAccumulator,
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
