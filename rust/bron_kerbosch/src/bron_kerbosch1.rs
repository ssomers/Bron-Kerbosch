//! Naive Bron-Kerbosch algorithm

use graph::{connected_nodes, vertex_set_with_capacity, UndirectedGraph, VertexSet};
use reporter::{Clique, Reporter};
use util::{intersect, pop_arbitrary};

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates = connected_nodes(graph);
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            reporter,
            candidates,
            vertex_set_with_capacity(num_candidates),
            Clique::new(),
        );
    }
}

fn visit(
    graph: &UndirectedGraph,
    reporter: &mut Reporter,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Clique,
) {
    debug_assert!(candidates.iter().all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.iter().all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));

    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(clique);
        return;
    }
    while let Some(v) = pop_arbitrary(&mut candidates) {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates = intersect(&neighbours, &candidates).cloned().collect();
        let neighbouring_excluded = intersect(&neighbours, &excluded).cloned().collect();
        visit(
            graph,
            reporter,
            neighbouring_candidates,
            neighbouring_excluded,
            [clique.as_slice(), &[v]].concat(),
        );
        excluded.insert(v);
    }
}
