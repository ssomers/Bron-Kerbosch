//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::clique::CliqueConsumer;
use super::graph::{UndirectedGraph, VertexSetLike, connected_vertices};
use super::pile::Pile;

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
            PivotChoice::MaxDegreeLocal,
            candidates,
            Graph::VertexSet::new(),
            &Pile::EMPTY,
        );
    }
}
