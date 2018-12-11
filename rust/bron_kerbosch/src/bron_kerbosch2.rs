//! Bron-Kerbosch algorithm with pivot

use graph::{UndirectedGraph, Vertex};
use reporter::{Clique, Reporter};

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates = graph.connected_nodes();
    if !candidates.is_empty() {
        visit(graph, reporter, candidates, HashSet::new(), Clique::new());
    }
}

pub fn visit(
    graph: &UndirectedGraph,
    reporter: &mut Reporter,
    mut candidates: HashSet<Vertex>,
    mut excluded: HashSet<Vertex>,
    clique: Clique,
) {
    debug_assert!(candidates.iter().all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.iter().all(|&v| graph.degree(v) > 0));
    reporter.inc_count();
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(clique);
        return;
    }

    let pivot = pick_arbitrary(if !candidates.is_empty() {
        &candidates
    } else {
        &excluded
    });
    let far_candidates: HashSet<Vertex> = candidates
        .difference(graph.adjacencies(pivot))
        .cloned()
        .collect();
    for v in far_candidates {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates = neighbours.intersection(&candidates).cloned().collect();
        let neighbouring_excluded = neighbours.intersection(&excluded).cloned().collect();
        visit(
            graph,
            reporter,
            neighbouring_candidates,
            neighbouring_excluded,
            [clique.as_slice(), &[v]].concat(),
        );
        candidates.remove(&v);
        excluded.insert(v);
    }
}

fn pick_arbitrary(s: &HashSet<Vertex>) -> Vertex {
    s.iter().next().unwrap().clone()
}
