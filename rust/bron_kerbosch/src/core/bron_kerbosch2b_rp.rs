//! Bron-Kerbosch algorithm with pivot picked randomly (IK_RP)

use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;
use crate::core::pile::Pile;

pub fn explore<VertexSet>(graph: &Graph<VertexSet>, mut consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
{
    let candidates: VertexSet = graph.connected_vertices().collect();
    if !candidates.is_empty() {
        visit(
            graph,
            &mut consumer,
            PivotChoice::Random,
            candidates,
            VertexSet::new(),
            &Pile::EMPTY,
        );
    }
}
