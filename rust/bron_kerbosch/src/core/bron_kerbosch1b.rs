//! Naive Bron-Kerbosch algorithm, optimized

use super::clique::CliqueConsumer;
use super::graph::{UndirectedGraph, Vertex, VertexSetLike, connected_vertices};
use super::pile::Pile;

type CliqueInProgress<'a> = Pile<'a, Vertex>;

pub fn explore<VertexSet, Graph>(graph: &Graph, mut consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    let candidates: VertexSet = connected_vertices(graph).collect();
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

fn visit<VertexSet, Graph>(
    graph: &Graph,
    consumer: &mut CliqueConsumer,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique_in_progress: &CliqueInProgress,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
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
                consumer,
                neighbouring_candidates,
                excluded.intersection_collect(neighbours),
                &clique_in_progress.pile(v),
            );
        } else if excluded.is_disjoint(neighbours) {
            consumer.accept(clique_in_progress.pile(v).collect());
        }
        excluded.insert(v);
    }
}
