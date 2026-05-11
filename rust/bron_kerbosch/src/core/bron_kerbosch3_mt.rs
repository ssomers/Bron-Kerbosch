//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)
//! implemented by multiple threads

use super::algorithm::BronKerboschAlgorithm;
use super::bron_kerbosch_degen_mt::{PivotChoice, explore_with_degeneracy_mt};
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub struct Algo<const VISITING_THREADS: usize>();
impl<const N: usize> BronKerboschAlgorithm for Algo<N> {
    fn name() -> String {
        format!("Ver3½=GP{N}")
    }

    fn deterministic() -> bool {
        false
    }

    fn explore<VertexSet, Consumer>(
        graph: &Graph<VertexSet>,
        consumer: Consumer,
    ) -> Consumer::Harvest
    where
        VertexSet: VertexSetLike + Sync,
        Consumer: CliqueConsumer + Clone + Send,
    {
        explore_with_degeneracy_mt(graph, consumer, PivotChoice::MaxDegreeLocal, N)
    }
}
