//! Naive Bron-Kerbosch algorithm, optimized

use graph::{connected_nodes, UndirectedGraph, Vertex};
use pile::Pile;
use reporter::Reporter;
use util::pop_arbitrary;

use std::collections::HashSet;

type Clique<'a> = Pile<'a, Vertex>;

pub fn explore(graph: &impl UndirectedGraph, reporter: &mut Reporter) {
    let candidates = connected_nodes(graph);
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            reporter,
            candidates,
            HashSet::with_capacity(num_candidates),
            Pile::new(),
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
        reporter.record(clique.collect());
        return;
    }

    while let Some(v) = pop_arbitrary(&mut candidates) {
        let neighbouring_candidates = graph.neighbour_intersection(&candidates, v);
        let neighbouring_excluded = graph.neighbour_intersection(&excluded, v);
        visit(
            graph,
            reporter,
            neighbouring_candidates,
            neighbouring_excluded,
            clique.cons(v),
        );
        excluded.insert(v);
    }
}
