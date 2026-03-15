//! Bron-Kerbosch algorithm with pivot picked randomly (IK_RP)

use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::clique::CliqueConsumer;
use super::graph::{UndirectedGraph, VertexSetLike, connected_vertices};
use crate::core::pile::Pile;

pub fn explore<VertexSet, Graph>(graph: &Graph, mut consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    let candidates: VertexSet = connected_vertices(graph).collect();
    if !candidates.is_empty() {
        visit(
            graph,
            &mut consumer,
            PivotChoice::Random,
            candidates,
            Graph::VertexSet::new(),
            &Pile::EMPTY,
        );
    }
}
