//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

use super::bron_kerbosch_degen::explore_with_degeneracy;
use crate::core::bron_kerbosch_pivot::PivotChoice;
use crate::{BronKerboschAlgorithm, CliqueAccumulator, Graph, VertexSetLike};

pub struct Algo();
impl BronKerboschAlgorithm for Algo {
    fn name() -> String {
        String::from("Ver3½-GPX")
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
        explore_with_degeneracy(
            graph,
            min_clique_size,
            accumulator,
            PivotChoice::MaxDegreeLocalX,
        )
    }
}
