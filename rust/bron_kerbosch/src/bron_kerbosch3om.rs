//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use bron_kerbosch_degeneracy::degeneracy_order_smart;
use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_nodes, UndirectedGraph, Vertex};
use pile::Pile;
use reporter::{Clique, Reporter};
use util::intersect;

extern crate crossbeam;
use std::collections::HashSet;
use std::sync::mpsc;

struct SendingReporter {
    tx: mpsc::Sender<Clique>,
}

impl Reporter for SendingReporter {
    fn record(&mut self, clique: Clique) {
        self.tx.send(clique).unwrap();
    }
}

struct VisitJob<'a> {
    candidates: HashSet<Vertex>,
    excluded: HashSet<Vertex>,
    clique: Pile<'a, Vertex>,
}

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    const NUM_THREADS: usize = 3;
    crossbeam::thread::scope(|scope| {
        let (reporter_tx, reporter_rx) = mpsc::channel();
        let mut job_txs: Vec<mpsc::Sender<VisitJob>> = Vec::with_capacity(NUM_THREADS);
        for _ in 0..NUM_THREADS {
            let (job_tx, thread_job_rx) = mpsc::channel();
            job_txs.push(job_tx);
            let thread_reporter_tx = reporter_tx.clone();
            scope.spawn(move |_| {
                let mut thread_reporter = SendingReporter {
                    tx: thread_reporter_tx,
                };
                while let Ok(job) = thread_job_rx.recv() {
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

        let mut candidates = connected_nodes(graph);
        debug_assert_eq!(
            degeneracy_order_smart(graph, &candidates).collect::<HashSet<_>>(),
            candidates
        );
        let mut excluded = HashSet::with_capacity(candidates.len());
        for (i, v) in degeneracy_order_smart(graph, &candidates).enumerate() {
            let neighbours = graph.adjacencies(v);
            debug_assert!(!neighbours.is_empty());
            candidates.remove(&v);
            let neighbouring_candidates = intersect(&neighbours, &candidates).cloned().collect();
            let neighbouring_excluded = intersect(&neighbours, &excluded).cloned().collect();
            excluded.insert(v);
            job_txs[i % NUM_THREADS]
                .send(VisitJob {
                    candidates: neighbouring_candidates,
                    excluded: neighbouring_excluded,
                    clique: Pile::from(v),
                })
                .unwrap();
        }
        drop(job_txs);
        while let Ok(clique) = reporter_rx.recv() {
            reporter.record(clique);
        }
    })
    .unwrap();
}
