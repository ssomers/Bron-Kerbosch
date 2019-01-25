//! Bron-Kerbosch algorithm with pivot picked arbitrarily

use graph::{connected_nodes, UndirectedGraph, Vertex};
use reporter::{Clique, Reporter};

use std::collections::HashSet;

pub fn explore(graph: &impl UndirectedGraph, reporter: &mut Reporter) {
    let candidates = connected_nodes(graph);
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            reporter,
            candidates,
            HashSet::with_capacity(num_candidates),
            Clique::new(),
        );
    }
}

pub fn visit(
    graph: &impl UndirectedGraph,
    reporter: &mut Reporter,
    mut candidates: HashSet<Vertex>,
    mut excluded: HashSet<Vertex>,
    clique: Clique,
) {
    debug_assert!(candidates.iter().all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.iter().all(|&v| graph.degree(v) > 0));
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(clique);
        return;
    }

    let pivot = pick_arbitrary(if !candidates.is_empty() {
        &candidates
    } else {
        &excluded
    });
    let mut pivot_neighbours: HashSet<Vertex> =
        HashSet::with_capacity(graph.degree(pivot) as usize);
    graph.visit_neighbours(pivot, |v| {
        pivot_neighbours.insert(v);
    });
    let far_candidates: HashSet<Vertex> =
        candidates.difference(&pivot_neighbours).cloned().collect();
    for v in far_candidates {
        let neighbouring_candidates = graph
            .neighbour_intersection(v, &candidates)
            .cloned()
            .collect();
        let neighbouring_excluded = graph
            .neighbour_intersection(v, &excluded)
            .cloned()
            .collect();
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
