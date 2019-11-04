//! Naive Bron-Kerbosch algorithm

use graph::{connected_vertices, UndirectedGraph, VertexSetLike};
use reporter::{Clique, Reporter};

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    let candidates = connected_vertices(graph);
    if !candidates.is_empty() {
        visit(graph, reporter, candidates, VertexSet::new(), Clique::new());
    }
}

fn visit<VertexSet, Graph, Rprtr>(
    graph: &Graph,
    reporter: &mut Rprtr,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Clique,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
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
    while let Some(v) = candidates.pop_arbitrary() {
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates = neighbours.intersection_collect(&candidates);
        let neighbouring_excluded = neighbours.intersection_collect(&excluded);
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
