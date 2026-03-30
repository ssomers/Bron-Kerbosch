//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::pile::Pile;
use super::vertexsetlike::VertexSetLike;

pub fn explore<VertexSet>(graph: &Graph<VertexSet>, mut consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
{
    let candidates: VertexSet = graph.connected_vertices().collect();
    if !candidates.is_empty() {
        visit(
            graph,
            &mut consumer,
            PivotChoice::MaxDegreeLocal,
            candidates,
            VertexSet::new(),
            &Pile::EMPTY,
        );
    }
}
