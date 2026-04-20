//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)
//! implemented by multiple threads

use super::bron_kerbosch_degen_mt::{PivotChoice, explore_with_degeneracy_mt};
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub fn explore<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    consumer: Consumer,
    num_visiting_threads: usize,
) -> Consumer::Harvest
where
    VertexSet: VertexSetLike + Sync,
    Consumer: CliqueConsumer + Clone + Send,
{
    explore_with_degeneracy_mt(
        graph,
        consumer,
        PivotChoice::MaxDegreeLocal,
        num_visiting_threads,
    )
}
