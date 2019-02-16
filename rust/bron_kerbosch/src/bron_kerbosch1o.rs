//! Naive Bron-Kerbosch algorithm, optimized

use graph::{connected_nodes, UndirectedGraph, Vertex, VertexSet};
use pile::Pile;
use reporter::Reporter;
use util::{intersect, pop_arbitrary};

type Clique<'a> = Pile<'a, Vertex>;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates = connected_nodes(graph);
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            reporter,
            candidates,
            VertexSet::with_capacity(num_candidates),
            Pile::new(),
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

    if candidates.is_empty() {
        if excluded.is_empty() {
            reporter.record(clique.collect());
        }
        return;
    }
    while let Some(v) = pop_arbitrary(&mut candidates) {
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates = intersect(&neighbours, &candidates).cloned().collect();
        let neighbouring_excluded = intersect(&neighbours, &excluded).cloned().collect();
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
