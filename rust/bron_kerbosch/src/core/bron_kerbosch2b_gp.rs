//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::bron_kerbosch_pivot::{PivotChoice, explore_with_pivot};
use crate::{BronKerboschAlgorithm, CliqueAccumulator, Graph, VertexSetLike};

pub struct Algo();
impl BronKerboschAlgorithm for Algo {
    fn name() -> String {
        String::from("Ver2½-GP")
    }

    fn explore<VertexSet, Accumulator>(
        graph: &Graph<VertexSet>,
        min_clique_size: usize,
        accumulator: Accumulator,
    ) -> Accumulator::Harvest
    where
        VertexSet: VertexSetLike,
        Accumulator: CliqueAccumulator,
    {
        explore_with_pivot(
            graph,
            min_clique_size,
            accumulator,
            PivotChoice::MaxDegreeLocal,
        )
    }
}
