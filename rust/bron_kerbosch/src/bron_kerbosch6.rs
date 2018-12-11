//! Bron-Kerbosch algorithm with pivot, slightly optimized and picking pivot smartly (IK_GPX)

use graph::{UndirectedGraph, Vertex};
use reporter::{Clique, Reporter};

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates: HashSet<Vertex> = graph.connected_nodes();
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

    let pivot = pick_best(graph, &candidates, &excluded);
    let far_candidates: Vec<Vertex> = candidates
        .difference(graph.adjacencies(pivot))
        .cloned()
        .collect();
    excluded.reserve(far_candidates.len());
    for v in far_candidates {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        candidates.remove(&v);
        let neighbouring_candidates: HashSet<Vertex> =
            neighbours.intersection(&candidates).cloned().collect();
        let neighbouring_excluded: HashSet<Vertex> =
            neighbours.intersection(&excluded).cloned().collect();
        excluded.insert(v);
        visit(
            graph,
            reporter,
            neighbouring_candidates,
            neighbouring_excluded,
            [clique.as_slice(), &[v]].concat(),
        );
    }
}

fn pick_best(
    graph: &UndirectedGraph,
    candidates: &HashSet<Vertex>,
    excluded: &HashSet<Vertex>,
) -> Vertex {
    debug_assert!(!(candidates.is_empty() && excluded.is_empty()));
    *candidates
        .iter()
        .chain(excluded)
        .max_by_key(|&&v| graph.adjacencies(v).intersection(&candidates).count())
        .unwrap()
}
