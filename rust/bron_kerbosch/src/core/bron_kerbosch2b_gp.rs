//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::bron_kerbosch_pivot::{PivotChoice, explore_with_pivot};
use super::clique::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub fn explore<VertexSet>(graph: &Graph<VertexSet>, consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
{
    explore_with_pivot(graph, consumer, PivotChoice::MaxDegreeLocal)
}
