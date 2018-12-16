//! Bron-Kerbosch algorithm with pivot, slightly optimized and picking pivot
//! with highest degree towards the remaining candidates (IK_GPX)

use graph::{UndirectedGraph, Vertex};
use reporter::{Clique, Reporter};

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let mut candidates = graph.connected_nodes();
    if !candidates.is_empty() {
        reporter.inc_count();
        let pivot = pick_max_degree(graph, candidates.iter().cloned());
        let far_candidates: Vec<Vertex> = candidates
            .difference(graph.adjacencies(pivot))
            .cloned()
            .collect();
        let mut excluded: HashSet<Vertex> = HashSet::new();
        excluded.reserve(far_candidates.len());
        for v in far_candidates {
            let neighbours = graph.adjacencies(v);
            debug_assert!(!neighbours.is_empty());
            candidates.remove(&v);
            let neighbouring_candidates = neighbours.intersection(&candidates).cloned().collect();
            let neighbouring_excluded = neighbours.intersection(&excluded).cloned().collect();
            excluded.insert(v);
            visit(
                graph,
                reporter,
                neighbouring_candidates,
                neighbouring_excluded,
                vec![v],
            );
        }
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
        let neighbouring_candidates = neighbours.intersection(&candidates).cloned().collect();
        let neighbouring_excluded = neighbours.intersection(&excluded).cloned().collect();
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

fn pick_max_degree(graph: &UndirectedGraph, vertices: impl Iterator<Item = Vertex>) -> Vertex {
    vertices.max_by_key(|&v| graph.degree(v)).unwrap()
}

fn pick_best(
    graph: &UndirectedGraph,
    candidates: &HashSet<Vertex>,
    excluded: &HashSet<Vertex>,
) -> Vertex {
    debug_assert!(!(candidates.is_empty() && excluded.is_empty()));
    candidates
        .iter()
        .chain(excluded)
        .cloned()
        .max_by_key(|&v| graph.adjacencies(v).intersection(&candidates).count())
        .unwrap()
}
