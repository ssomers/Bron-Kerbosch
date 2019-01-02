//! Naive Bron-Kerbosch algorithm, optimized

use graph::{UndirectedGraph, Vertex};
use reporter::Reporter;
use util::intersect;
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

fn visit(
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

    while let Some(v) = remove_arbitrary(&mut candidates) {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates = intersect(&neighbours, &candidates).cloned().collect();
        let neighbouring_excluded = intersect(neighbours, &excluded).cloned().collect();
        visit(
            graph,
            reporter,
            neighbouring_candidates,
            neighbouring_excluded,
            VertexStack::Cons(&clique, v),
        );
        excluded.insert(v);
    }
}

fn remove_arbitrary(s: &mut HashSet<Vertex>) -> Option<Vertex> {
    s.iter().next().cloned().map(|v| {
        s.remove(&v);
        v
    })
}
