//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)
//! implemented by multiple threads

use super::bron_kerbosch_degen_mt::explore_with_degeneracy_mt;
use crate::core::bron_kerbosch_pivot::PivotChoice;
use crate::{BronKerboschAlgorithm, CliqueAccumulator, Graph, VertexSetLike};

pub struct Algo<const VISITING_THREADS: usize>();
impl<const N: usize> BronKerboschAlgorithm for Algo<N> {
    fn name() -> String {
        format!("Ver3½=GP{N}")
    }

    fn deterministic() -> bool {
        false
    }

    fn explore<VertexSet, Accumulator>(
        graph: &Graph<VertexSet>,
        min_clique_size: usize,
        accumulator: Accumulator,
    ) -> Accumulator::Harvest
    where
        VertexSet: VertexSetLike + Sync,
        Accumulator: CliqueAccumulator + Clone + Send,
    {
        explore_with_degeneracy_mt(
            graph,
            min_clique_size,
            accumulator,
            PivotChoice::MaxDegreeLocal,
            N,
        )
    }
}
