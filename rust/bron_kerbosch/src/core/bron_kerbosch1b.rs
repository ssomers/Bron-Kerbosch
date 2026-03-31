//! Naive Bron-Kerbosch algorithm, optimized

use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::pile::Pile;
use super::vertex::Vertex;
use super::vertexsetlike::VertexSetLike;
use std::ops::Not;

type CliqueInProgress<'a> = Pile<'a, Vertex>;

pub fn explore<VertexSet>(graph: &Graph<VertexSet>, mut consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
{
    let candidates: VertexSet = graph.connected_vertices().collect();
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            &mut consumer,
            candidates,
            VertexSet::with_capacity(num_candidates),
            &Pile::EMPTY,
        );
    }
}

fn visit<VertexSet>(
    graph: &Graph<VertexSet>,
    consumer: &mut CliqueConsumer,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique_in_progress: &CliqueInProgress,
) where
    VertexSet: VertexSetLike,
{
    debug_assert!(candidates.all(|&v| graph.is_connected(v)));
    debug_assert!(excluded.all(|&v| graph.is_connected(v)));
    debug_assert!(candidates.is_disjoint(&excluded));
    debug_assert!(candidates.is_empty().not());

    while let Some(v) = candidates.pop_arbitrary() {
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates: VertexSet = candidates.intersection_collect(neighbours);
        if neighbouring_candidates.is_empty().not() {
            visit(
                graph,
                consumer,
                neighbouring_candidates,
                excluded.intersection_collect(neighbours),
                &clique_in_progress.pile(v),
            );
        } else if consumer.is_accepted_size(clique_in_progress.height + 1)
            && excluded.is_disjoint(neighbours)
        {
            consumer.accept(clique_in_progress.pile(v).iter().copied().collect());
        }
        excluded.insert(v);
    }
}
