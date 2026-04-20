//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)

use super::bron_kerbosch_degen::{PivotChoice, explore_with_degeneracy};
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub fn explore<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    consumer: Consumer,
) -> Consumer::Harvest
where
    VertexSet: VertexSetLike,
    Consumer: CliqueConsumer,
{
    explore_with_degeneracy(graph, consumer, PivotChoice::MaxDegreeLocal)
}
