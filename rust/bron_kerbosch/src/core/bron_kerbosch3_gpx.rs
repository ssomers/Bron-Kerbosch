//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

use super::bron_kerbosch_degen::{PivotChoice, explore_with_pivot};
use super::clique::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub fn explore<VertexSet>(graph: &Graph<VertexSet>, consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
{
    explore_with_pivot(graph, consumer, PivotChoice::MaxDegreeLocalX)
}
