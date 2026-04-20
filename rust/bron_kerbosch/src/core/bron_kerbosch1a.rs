//! Naive Bron-Kerbosch algorithm

use super::clique::Clique;
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;
use std::ops::Not;

pub fn explore<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    mut consumer: Consumer,
) -> Consumer::Harvest
where
    VertexSet: VertexSetLike,
    Consumer: CliqueConsumer,
{
    let candidates: VertexSet = graph.connected_vertices().collect();
    if candidates.is_empty().not() {
        visit(
            graph,
            &mut consumer,
            candidates,
            VertexSet::new(),
            Clique::EMPTY,
        );
    }
    consumer.harvest()
}

fn visit<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    consumer: &mut Consumer,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique_in_progress: Clique,
) where
    VertexSet: VertexSetLike,
    Consumer: CliqueConsumer,
{
    debug_assert!(candidates.all(|&v| graph.is_connected(v)));
    debug_assert!(excluded.all(|&v| graph.is_connected(v)));
    debug_assert!(candidates.is_disjoint(&excluded));

    if candidates.is_empty() {
        if excluded.is_empty() && consumer.is_accepted_size(clique_in_progress.len()) {
            consumer.accept(clique_in_progress);
        }
        return;
    }
    while let Some(v) = candidates.pop_arbitrary() {
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates = neighbours.intersection(&candidates).copied().collect();
        let neighbouring_excluded = neighbours.intersection(&excluded).copied().collect();
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
