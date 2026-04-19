//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

use super::bron_kerbosch_degen::{PivotChoice, explore_with_degeneracy};
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub fn explore<VertexSet>(graph: &Graph<VertexSet>, consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
{
    explore_with_degeneracy(graph, consumer, PivotChoice::MaxDegreeLocalX)
}
