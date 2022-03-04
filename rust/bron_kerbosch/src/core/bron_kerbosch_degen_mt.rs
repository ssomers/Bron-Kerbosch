//! Core of Bron-Kerbosch algorithms using degeneracy ordering and multiple threads.

use super::bron_kerbosch_pivot::visit;
pub use super::bron_kerbosch_pivot::PivotChoice;
use super::graph::{UndirectedGraph, Vertex, VertexSetLike};
use super::graph_degeneracy::degeneracy_ordering;
use super::pile::Pile;
use super::reporter::{Clique, Reporter};
use crossbeam_channel::{Receiver, Sender};

pub fn explore_with_pivot_multithreaded<VertexSet, Graph, Rprtr>(
    graph: &Graph,
    reporter: &mut Rprtr,
    pivot_selection: PivotChoice,
    num_visiting_threads: usize,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    crossbeam::thread::scope(|scope| {
        let (start_tx, start_rx) = crossbeam_channel::bounded(64);
        let (visit_tx, visit_rx) = crossbeam_channel::bounded(64);
        let (reporter_tx, reporter_rx) = crossbeam_channel::bounded(64);

        scope.spawn(move |_| initiate(graph, start_tx));
        scope.spawn(move |_| dispatch(graph, start_rx, visit_tx));

        for _ in 0..num_visiting_threads {
            let thread_visit_rx = visit_rx.clone();
            let thread_reporter_tx = reporter_tx.clone();
            scope.spawn(move |_| {
                descend(graph, pivot_selection, thread_visit_rx, thread_reporter_tx)
            });
        }
        drop(visit_rx);
        drop(reporter_tx);

        while let Ok(clique) = reporter_rx.recv() {
            reporter.record(clique);
        }
    })
    .unwrap();
}

struct VisitJob<VertexSet> {
    start: Vertex,
    candidates: VertexSet,
    excluded: VertexSet,
}

fn initiate<VertexSet, Graph>(graph: &Graph, start_tx: Sender<Vertex>)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    for vertex in degeneracy_ordering(graph, -1) {
        start_tx.send(vertex).unwrap();
    }
}

fn dispatch<VertexSet, Graph>(
    graph: &Graph,
    start_rx: Receiver<Vertex>,
    visit_tx: Sender<VisitJob<VertexSet>>,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    // In this initial iteration, we don't need to represent the set of candidates
    // because all neighbours are candidates until excluded.
    let mut excluded = VertexSet::with_capacity(graph.order());
    while let Ok(v) = start_rx.recv() {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_excluded: VertexSet = neighbours.intersection_collect(&excluded);
        if neighbouring_excluded.len() < neighbours.len() {
            let neighbouring_candidates: VertexSet =
                neighbours.difference_collect(&neighbouring_excluded);
            let visit = VisitJob {
                start: v,
                candidates: neighbouring_candidates,
                excluded: neighbouring_excluded,
            };
            visit_tx.send(visit).unwrap();
        }
        excluded.insert(v);
    }
}

fn descend<VertexSet, Graph>(
    graph: &Graph,
    pivot_selection: PivotChoice,
    visit_rx: Receiver<VisitJob<VertexSet>>,
    report_tx: Sender<Clique>,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    struct SendingReporter(Sender<Clique>);
    impl Reporter for SendingReporter {
        fn record(&mut self, clique: Clique) {
            self.0.send(clique).unwrap();
        }
    }
    let mut reporter = SendingReporter(report_tx);

    while let Ok(job) = visit_rx.recv() {
        visit(
            graph,
            &mut reporter,
            pivot_selection,
            job.candidates,
            job.excluded,
            Some(&Pile::from(job.start)),
        );
    }
}
