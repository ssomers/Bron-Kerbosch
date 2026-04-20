//! Bron-Kerbosch algorithm with pivot picked randomly (IK_RP)

use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::pile::Pile;
use super::vertexsetlike::VertexSetLike;
use std::ops::Not;

pub fn explore<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    mut consumer: Consumer,
) -> Consumer::Harvest
where
    VertexSet: VertexSetLike,
    Consumer: CliqueConsumer,
{
    let candidates: VertexSet = graph.connected_vertices().collect();
    if candidates.is_empty().not() {
        visit(
            graph,
            &mut consumer,
            PivotChoice::Random,
            candidates,
            VertexSet::new(),
            &Pile::EMPTY,
        );
    }
    consumer.harvest()
}
