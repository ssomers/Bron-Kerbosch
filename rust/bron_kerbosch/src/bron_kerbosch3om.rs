//! Bron-Kerbosch algorithm with degeneracy ordering,
//! recursing with pivot of highest degree (IK_GP)
//! implemented by multiple threads

use super::mpmc;
use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{UndirectedGraph, Vertex, VertexSetLike};
use graph_degeneracy::degeneracy_ordering;
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

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike + Send,
    Graph: UndirectedGraph<VertexSet>,
    Rprtr: Reporter,
{
    const NUM_VISITING_THREADS: usize = 3;

    crossbeam::thread::scope(|scope| {
        let (start_tx, start_rx) = mpsc::channel();
        let (visit_tx, visit_rx) = mpmc::channel::<VisitJob<VertexSet>>();
        let (reporter_tx, reporter_rx) = mpsc::channel();

        scope.spawn(move |_| {
            for vertex in degeneracy_ordering(graph, -1) {
                start_tx.send(vertex).unwrap();
            }
        });

        scope.spawn(move |_| {
            let mut excluded = VertexSet::new();
            while let Ok(v) = start_rx.recv() {
                let neighbours = graph.neighbours(v);
                debug_assert!(!neighbours.is_empty());
                let neighbouring_candidates: VertexSet = neighbours.difference(&excluded);
                if neighbouring_candidates.is_empty() {
                    debug_assert!(!neighbours.is_disjoint(&excluded));
                } else {
                    let neighbouring_excluded: VertexSet = neighbours.intersection(&excluded);
                    visit_tx
                        .send(VisitJob {
                            candidates: neighbouring_candidates,
                            excluded: neighbouring_excluded,
                            clique: Pile::from(v),
                        })
                        .unwrap();
                }
                excluded.insert(v);
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
