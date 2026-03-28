//! Naive Bron-Kerbosch algorithm

use super::clique::{Clique, CliqueConsumer};
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub fn explore<VertexSet>(graph: &Graph<VertexSet>, mut consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
{
    let candidates: VertexSet = graph.connected_vertices().collect();
    if !candidates.is_empty() {
        visit(
            graph,
            &mut consumer,
            candidates,
            VertexSet::new(),
            Clique::new(),
        );
    }
}

fn visit<VertexSet>(
    graph: &Graph<VertexSet>,
    consumer: &mut CliqueConsumer,
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
        if excluded.is_empty() && consumer.is_accepted_size(clique.len()) {
            consumer.accept(clique);
        }
        return;
    }
    while let Some(v) = candidates.pop_arbitrary() {
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates = neighbours.intersection_collect(&candidates);
        let neighbouring_excluded = neighbours.intersection_collect(&excluded);
        visit(
            graph,
            consumer,
            neighbouring_candidates,
            neighbouring_excluded,
            [clique.as_slice(), &[v]].concat(),
        );
        excluded.insert(v);
    }
}
