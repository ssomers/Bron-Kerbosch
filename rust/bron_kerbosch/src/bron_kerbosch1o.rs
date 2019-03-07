//! Naive Bron-Kerbosch algorithm, optimized

use graph::{connected_nodes, UndirectedGraph, Vertex, VertexSetLike};
use pile::Pile;
use reporter::Reporter;

type Clique<'a> = Pile<'a, Vertex>;

pub fn explore<VertexSet>(graph: &UndirectedGraph<VertexSet>, reporter: &mut Reporter)
where
    VertexSet: VertexSetLike<VertexSet>,
{
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

fn visit<VertexSet>(
    graph: &UndirectedGraph<VertexSet>,
    reporter: &mut Reporter,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Clique,
) where
    VertexSet: VertexSetLike<VertexSet>,
{
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));

    if candidates.is_empty() {
        if excluded.is_empty() {
            reporter.record(clique.collect());
        }
        return;
    }
    while let Some(v) = candidates.pop_arbitrary() {
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates = neighbours.intersection(&candidates);
        let neighbouring_excluded = neighbours.intersection(&excluded);
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
