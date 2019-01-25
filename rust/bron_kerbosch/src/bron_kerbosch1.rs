//! Naive Bron-Kerbosch algorithm

use graph::{connected_nodes, UndirectedGraph, Vertex};
use reporter::{Clique, Reporter};
use util::pop_arbitrary;

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

fn visit(
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

    while let Some(v) = pop_arbitrary(&mut candidates) {
        let neighbouring_candidates: HashSet<Vertex> = graph
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
        excluded.insert(v);
    }
}
