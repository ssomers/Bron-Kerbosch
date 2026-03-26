//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::bron_kerbosch_pivot::{PivotChoice, explore_with_pivot};
use super::clique::CliqueConsumer;
use super::graphlike::{GraphLike, VertexSetLike};

pub fn explore<VertexSet, Graph>(graph: &Graph, consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
    Graph: GraphLike<VertexSet = VertexSet>,
{
    explore_with_pivot(graph, consumer, PivotChoice::MaxDegreeLocal)
}
