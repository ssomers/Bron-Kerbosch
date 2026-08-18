//! Naive Bron-Kerbosch algorithm

use super::clique_consumer::CliqueConsumer;
use crate::{BronKerboschAlgorithm, Clique, CliqueAccumulator, Graph, VertexSetLike};
use std::ops::Not;

pub struct Algo();
impl BronKerboschAlgorithm for Algo {
    fn name() -> String {
        String::from("Ver1")
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
        if candidates.is_empty().not() {
            let mut consumer = CliqueConsumer {
                min_clique_size,
                accu: &mut accumulator,
            };
            visit(
                graph,
                &mut consumer,
                candidates,
                VertexSet::new(),
                Clique::EMPTY,
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
    clique_in_progress: Clique,
) where
    VertexSet: VertexSetLike,
    Accumulator: CliqueAccumulator,
{
    debug_assert!(candidates.all(|&v| graph.is_connected(v)));
    debug_assert!(excluded.all(|&v| graph.is_connected(v)));
    debug_assert!(candidates.is_disjoint(&excluded));

    if candidates.is_empty() {
        if excluded.is_empty() && clique_in_progress.len() >= consumer.min_clique_size {
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
