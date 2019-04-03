//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use super::mpmc;
use bron_kerbosch_degeneracy::degeneracy_order;
use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_nodes, UndirectedGraph, Vertex, VertexSetLike};
use pile::Pile;
use reporter::{Clique, Reporter};

use std::sync::mpsc;

struct SendingReporter {
    tx: mpsc::Sender<Clique>,
}

impl Reporter for SendingReporter {
    fn record(&mut self, clique: Clique) {
        self.tx.send(clique).unwrap();
    }
}

struct VisitJob<'a, VertexSet> {
    candidates: VertexSet,
    excluded: VertexSet,
    clique: Pile<'a, Vertex>,
}

pub fn explore<VertexSet>(graph: &UndirectedGraph<VertexSet>, reporter: &mut Reporter)
where
    VertexSet: VertexSetLike + Send,
{
    const NUM_VISITING_THREADS: usize = 3;

    crossbeam::thread::scope(|scope| {
        let (start_tx, start_rx) = mpsc::channel();
        let (visit_tx, visit_rx) = mpmc::channel::<VisitJob<VertexSet>>();
        let (reporter_tx, reporter_rx) = mpsc::channel();

        scope.spawn(move |_| {
            for vertex in degeneracy_order(graph) {
                start_tx.send(vertex).unwrap();
            }
        });

        scope.spawn(move |_| {
            let mut candidates = connected_nodes(graph);
            let mut excluded = VertexSet::with_capacity(candidates.len());
            while let Ok(v) = start_rx.recv() {
                let neighbours = graph.neighbours(v);
                debug_assert!(!neighbours.is_empty());
                candidates.remove(&v);
                let neighbouring_candidates = neighbours.intersection(&candidates);
                let neighbouring_excluded = neighbours.intersection(&excluded);
                excluded.insert(v);
                visit_tx
                    .send(VisitJob {
                        candidates: neighbouring_candidates,
                        excluded: neighbouring_excluded,
                        clique: Pile::from(v),
                    })
                    .unwrap();
            }
        });

        for _ in 0..NUM_VISITING_THREADS {
            let thread_visit_rx = visit_rx.clone();
            let thread_reporter_tx = reporter_tx.clone();
            scope.spawn(move |_| {
                let mut thread_reporter = SendingReporter {
                    tx: thread_reporter_tx,
                };
                while let Ok(job) = thread_visit_rx.recv() {
                    visit(
                        graph,
                        &mut thread_reporter,
                        PivotChoice::MaxDegree,
                        PivotChoice::MaxDegree,
                        job.candidates,
                        job.excluded,
                        job.clique,
                    );
                }
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
