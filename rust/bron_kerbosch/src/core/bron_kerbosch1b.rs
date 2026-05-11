//! Naive Bron-Kerbosch algorithm, optimized

use super::algorithm::BronKerboschAlgorithm;
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::pile::Pile;
use super::vertex::Vertex;
use super::vertexsetlike::VertexSetLike;
use std::ops::Not;

type CliqueInProgress<'a> = Pile<'a, Vertex>;

pub struct Algo();
impl BronKerboschAlgorithm for Algo {
    fn name() -> String {
        String::from("Ver1½")
    }

    fn explore<VertexSet, Consumer>(
        graph: &Graph<VertexSet>,
        mut consumer: Consumer,
    ) -> Consumer::Harvest
    where
        VertexSet: VertexSetLike,
        Consumer: CliqueConsumer,
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
        consumer.harvest()
    }
}

fn visit<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    consumer: &mut Consumer,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique_in_progress: &CliqueInProgress,
) where
    VertexSet: VertexSetLike,
    Consumer: CliqueConsumer,
{
    debug_assert!(candidates.all(|&v| graph.is_connected(v)));
    debug_assert!(excluded.all(|&v| graph.is_connected(v)));
    debug_assert!(candidates.is_disjoint(&excluded));
    debug_assert!(candidates.is_empty().not());

    while let Some(v) = candidates.pop_arbitrary() {
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates: VertexSet =
            candidates.intersection(neighbours).copied().collect();
        if neighbouring_candidates.is_empty().not() {
            visit(
                graph,
                consumer,
                neighbouring_candidates,
                excluded.intersection(neighbours).copied().collect(),
                &clique_in_progress.pile(v),
            );
        } else if clique_in_progress.height + 1 >= consumer.min_size()
            && excluded.is_disjoint(neighbours)
        {
            let clique = clique_in_progress.pile(v);
            consumer.accept(clique.iter().copied().collect());
        }
        excluded.insert(v);
    }
}
