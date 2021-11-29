//! Naive Bron-Kerbosch algorithm, optimized

use crate::graph::{connected_vertices, UndirectedGraph, Vertex, VertexSetLike};
use crate::pile::Pile;
use crate::reporter::Reporter;

type Clique<'a> = Pile<'a, Vertex>;

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    let candidates = connected_vertices(graph);
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            reporter,
            candidates,
            VertexSet::with_capacity(num_candidates),
            None,
        );
    }
}

fn visit<VertexSet, Graph, Rprtr>(
    graph: &Graph,
    reporter: &mut Rprtr,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Option<&Clique>,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));
    debug_assert!(!candidates.is_empty());

    while let Some(v) = candidates.pop_arbitrary() {
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates: VertexSet = candidates.intersection_collect(neighbours);
        if !neighbouring_candidates.is_empty() {
            visit(
                graph,
                reporter,
                neighbouring_candidates,
                excluded.intersection_collect(neighbours),
                Some(&Pile::on(clique, v)),
            );
        } else if excluded.is_disjoint(neighbours) {
            reporter.record(Pile::on(clique, v).collect());
        }
        excluded.insert(v);
    }
}
