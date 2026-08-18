//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::clique_consumer::CliqueConsumer;
use super::pile::Pile;
use crate::{BronKerboschAlgorithm, CliqueAccumulator, Graph, VertexSetLike};
use std::ops::Not;

pub struct Algo();
impl BronKerboschAlgorithm for Algo {
    fn name() -> String {
        String::from("Ver2-GP")
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
                PivotChoice::MaxDegreeLocal,
                candidates,
                VertexSet::new(),
                &Pile::EMPTY,
            );
        }
        accumulator.harvest()
    }
}
