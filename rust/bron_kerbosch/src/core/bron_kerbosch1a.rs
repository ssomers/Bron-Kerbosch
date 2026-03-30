//! Naive Bron-Kerbosch algorithm

use super::clique::Clique;
use super::clique_consumer::CliqueConsumer;
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
            Clique::EMPTY,
        );
    }
}

fn visit<VertexSet>(
    graph: &Graph<VertexSet>,
    consumer: &mut CliqueConsumer,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique_in_progress: Clique,
) where
    VertexSet: VertexSetLike,
{
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));

    if candidates.is_empty() {
        if excluded.is_empty() && consumer.is_accepted_size(clique_in_progress.len()) {
            consumer.accept(clique_in_progress);
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
            clique_in_progress.add(v),
        );
        excluded.insert(v);
    }
}
