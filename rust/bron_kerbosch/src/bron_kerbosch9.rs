//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use bron_kerbosch_degeneracy::degeneracy_order_smart;
use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::UndirectedGraph;
use pile::Pile;
use reporter::{Clique, Reporter};
use util::intersect;

extern crate crossbeam;
use std::collections::HashSet;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

struct SendingReporter {
    tx: Sender<Clique>,
}

impl Reporter for SendingReporter {
    fn record(&mut self, clique: Clique) {
        self.tx.send(clique).unwrap();
    }
}

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let mut candidates = graph.connected_nodes();
    debug_assert_eq!(
        degeneracy_order_smart(graph, &candidates).collect::<HashSet<_>>(),
        candidates
    );
    let mut excluded = HashSet::with_capacity(candidates.len());

    let (tx, rx) = mpsc::channel();
    crossbeam::thread::scope(|scope| {
        for v in degeneracy_order_smart(graph, &candidates) {
            let neighbours = graph.adjacencies(v);
            debug_assert!(!neighbours.is_empty());
            candidates.remove(&v);
            let neighbouring_candidates = intersect(&neighbours, &candidates).cloned().collect();
            let neighbouring_excluded = intersect(&neighbours, &excluded).cloned().collect();
            excluded.insert(v);
            let thread_tx = tx.clone();
            scope.spawn(move |_| {
                visit(
                    graph,
                    &mut SendingReporter { tx: thread_tx },
                    PivotChoice::MaxDegree,
                    PivotChoice::MaxDegree,
                    neighbouring_candidates,
                    neighbouring_excluded,
                    Pile::Cons(&Pile::Empty, v),
                );
            });
        }
        drop(tx);
        while let Ok(clique) = rx.recv() {
            reporter.record(clique);
        }
    })
    .unwrap();
}
