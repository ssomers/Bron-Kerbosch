//! Naive Bron-Kerbosch algorithm, optimized

use graph::{connected_vertices, UndirectedGraph, Vertex, VertexSetLike};
use pile::Pile;
use reporter::Reporter;

type Clique<'a> = Pile<'a, Vertex>;

pub fn explore<VertexSet>(graph: &UndirectedGraph<VertexSet>, reporter: &mut Reporter)
where
    VertexSet: VertexSetLike,
{
    let candidates = connected_vertices(graph);
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
    VertexSet: VertexSetLike,
{
    debug_assert!(!candidates.is_empty());
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));

    loop {
        let v = candidates.pop_arbitrary().unwrap();
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates: VertexSet = neighbours.intersection(&candidates);
        if !neighbouring_candidates.is_empty() {
            let neighbouring_excluded: VertexSet = neighbours.intersection(&excluded);
            visit(
                graph,
                reporter,
                neighbouring_candidates,
                neighbouring_excluded,
                clique.place(v),
            );
        } else {
            if neighbours.is_disjoint(&excluded) {
                reporter.record(clique.place(v).collect());
            }
            if candidates.is_empty() {
                break;
            }
        }
        excluded.insert(v);
    }
}
