//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::bron_kerbosch_pivot::{PivotChoice, explore_with_pivot};
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
    explore_with_pivot(graph, consumer, PivotChoice::MaxDegreeLocal)
}
