//! Naive Bron-Kerbosch algorithm, optimized

use super::clique_consumer::CliqueConsumer;
use super::pile::Pile;
use crate::{BronKerboschAlgorithm, CliqueAccumulator, Graph, Vertex, VertexSetLike};
use std::ops::Not;

type CliqueInProgress<'a> = Pile<'a, Vertex>;

pub struct Algo();
impl BronKerboschAlgorithm for Algo {
    fn name() -> String {
        String::from("Ver1½")
    }

    fn explore<VertexSet, Accumulator>(
        graph: &Graph<VertexSet>,
        min_clique_size: usize,
        mut accumulator: Accumulator,
    ) -> Accumulator::Harvest
    where
        VertexSet: VertexSetLike,
        Accumulator: CliqueAccumulator,
    {
        let candidates: VertexSet = graph.connected_vertices().collect();
        let num_candidates = candidates.len();
        if num_candidates > 0 {
            let mut consumer = CliqueConsumer {
                min_clique_size,
                accu: &mut accumulator,
            };
            visit(
                graph,
                &mut consumer,
                candidates,
                VertexSet::with_capacity(num_candidates),
                &Pile::EMPTY,
            );
        }
        accumulator.harvest()
    }
}

fn visit<VertexSet, Accumulator>(
    graph: &Graph<VertexSet>,
    consumer: &mut CliqueConsumer<Accumulator>,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique_in_progress: &CliqueInProgress,
) where
    VertexSet: VertexSetLike,
    Accumulator: CliqueAccumulator,
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
        } else if clique_in_progress.height + 1 >= consumer.min_clique_size
            && excluded.is_disjoint(neighbours)
        {
            let clique = clique_in_progress.pile(v);
            consumer.accept(clique.iter().copied().collect());
        }
        excluded.insert(v);
    }
}
