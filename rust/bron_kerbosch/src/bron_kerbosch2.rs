//! Bron-Kerbosch algorithm with pivot picked arbitrarily

use graph::{connected_nodes, UndirectedGraph, VertexSetLike};
use reporter::{Clique, Reporter};

pub fn explore<VertexSet>(graph: &UndirectedGraph<VertexSet>, reporter: &mut Reporter)
where
    VertexSet: VertexSetLike,
{
    let candidates = connected_nodes(graph);
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            reporter,
            candidates,
            VertexSet::with_capacity(num_candidates),
            Clique::new(),
        );
    }
}

pub fn visit<VertexSet>(
    graph: &UndirectedGraph<VertexSet>,
    reporter: &mut Reporter,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Clique,
) where
    VertexSet: VertexSetLike,
{
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));

    if candidates.is_empty() {
        if excluded.is_empty() {
            reporter.record(clique);
        }
        return;
    }

    let &pivot = candidates.choose_arbitrary().unwrap();
    let far_candidates = candidates.difference(graph.neighbours(pivot));
    for v in far_candidates {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates = neighbours.intersection(&candidates);
        let neighbouring_excluded = neighbours.intersection(&excluded);
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
