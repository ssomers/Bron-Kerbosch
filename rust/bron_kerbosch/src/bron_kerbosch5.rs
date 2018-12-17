//! Bron-Kerbosch algorithm with pivot, slightly optimized and picking pivot
//! with highest degree (IK_GP)

use graph::{UndirectedGraph, Vertex};
use reporter::Reporter;
use vertex_stack::VertexStack;

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates = graph.connected_nodes();
    if !candidates.is_empty() {
        visit(
            graph,
            reporter,
            candidates,
            HashSet::new(),
            VertexStack::Empty,
        );
    }
}

pub fn visit(
    graph: &UndirectedGraph,
    reporter: &mut Reporter,
    mut candidates: HashSet<Vertex>,
    mut excluded: HashSet<Vertex>,
    clique: VertexStack,
) {
    debug_assert!(candidates.iter().all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.iter().all(|&v| graph.degree(v) > 0));
    reporter.inc_count();
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(clique.collect());
        return;
    }

    let pivot = pick_max_degree(graph, candidates.iter().chain(&excluded).cloned());
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
            VertexStack::Cons(&clique, v),
        );
    }
}

fn pick_max_degree(graph: &UndirectedGraph, vertices: impl Iterator<Item = Vertex>) -> Vertex {
    vertices.max_by_key(|&v| graph.degree(v)).unwrap()
}
