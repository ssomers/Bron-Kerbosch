//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)
//! implemented by multiple threads

use crate::bron_kerbosch_pivot::{visit, PivotChoice};
use crate::graph::{UndirectedGraph, Vertex, VertexSetLike};
use crate::graph_degeneracy::degeneracy_ordering;
use crate::pile::Pile;
use crate::reporter::{Clique, Reporter};

struct SendingReporter {
    tx: crossbeam_channel::Sender<Clique>,
}

impl Reporter for SendingReporter {
    fn record(&mut self, clique: Clique) {
        self.tx.send(clique).unwrap();
    }
}

struct VisitJob<VertexSet> {
    start: Vertex,
    candidates: VertexSet,
    excluded: VertexSet,
}

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike + Send,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    const NUM_VISITING_THREADS: usize = 5;

    crossbeam::thread::scope(|scope| {
        let (start_tx, start_rx) = crossbeam_channel::bounded(64);
        let (visit_tx, visit_rx) = crossbeam_channel::bounded(64);
        let (reporter_tx, reporter_rx) = crossbeam_channel::bounded(64);

        scope.spawn(move |_| {
            for vertex in degeneracy_ordering(graph, -1) {
                start_tx.send(vertex).unwrap();
            }
        });

        scope.spawn(move |_| {
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            let mut excluded = VertexSet::new();
            while let Ok(v) = start_rx.recv() {
                let neighbours = graph.neighbours(v);
                debug_assert!(!neighbours.is_empty());
                let neighbouring_excluded: VertexSet = neighbours.intersection_collect(&excluded);
                if neighbouring_excluded.len() < neighbours.len() {
                    let neighbouring_candidates: VertexSet =
                        neighbours.difference_collect(&neighbouring_excluded);
                    visit_tx
                        .send(VisitJob {
                            start: v,
                            candidates: neighbouring_candidates,
                            excluded: neighbouring_excluded,
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
                        PivotChoice::MaxDegreeLocal,
                        PivotChoice::MaxDegreeLocal,
                        job.candidates,
                        job.excluded,
                        Some(&Pile::from(job.start)),
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
