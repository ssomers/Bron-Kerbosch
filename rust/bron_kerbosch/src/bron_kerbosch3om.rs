//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use bron_kerbosch_degeneracy::degeneracy_order_smart;
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

struct StartJob {
    vertex: Vertex,
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
        let (start_tx, start_rx): (mpsc::Sender<StartJob>, mpsc::Receiver<StartJob>) =
            mpsc::channel();
        let (reporter_tx, reporter_rx) = mpsc::channel();
        let mut visit_txs: Vec<mpsc::Sender<VisitJob<VertexSet>>> =
            Vec::with_capacity(NUM_VISITING_THREADS);
        for _ in 0..NUM_VISITING_THREADS {
            let (job_tx, job_rx) = mpsc::channel();
            visit_txs.push(job_tx);
            let thread_reporter_tx = reporter_tx.clone();
            scope.spawn(move |_| {
                let mut thread_reporter = SendingReporter {
                    tx: thread_reporter_tx,
                };
                while let Ok(job) = job_rx.recv() {
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
        drop(reporter_tx);

        scope.spawn(move |_| {
            let mut candidates = connected_nodes(graph);
            let mut excluded = VertexSet::with_capacity(candidates.len());
            while let Ok(job) = start_rx.recv() {
                let v = job.vertex;
                let i = excluded.len() % NUM_VISITING_THREADS;
                let neighbours = graph.neighbours(v);
                debug_assert!(!neighbours.is_empty());
                candidates.remove(&v);
                let neighbouring_candidates = neighbours.intersection(&candidates);
                let neighbouring_excluded = neighbours.intersection(&excluded);
                excluded.insert(v);
                visit_txs[i]
                    .send(VisitJob {
                        candidates: neighbouring_candidates,
                        excluded: neighbouring_excluded,
                        clique: Pile::from(v),
                    })
                    .unwrap();
            }
        });
        scope.spawn(move |_| {
            while let Ok(clique) = reporter_rx.recv() {
                reporter.record(clique);
            }
        });

        for vertex in degeneracy_order_smart(graph) {
            start_tx.send(StartJob { vertex }).unwrap();
        }
    })
    .unwrap();
}
