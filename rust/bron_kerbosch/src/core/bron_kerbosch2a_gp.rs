//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::base::CliqueConsumer;
use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::graph::{UndirectedGraph, VertexSetLike, connected_vertices};

pub fn explore<VertexSet, Graph, Consumer>(graph: &Graph, consumer: &mut Consumer)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Consumer: CliqueConsumer,
{
    let candidates: VertexSet = connected_vertices(graph).collect();
    if !candidates.is_empty() {
        visit(
            graph,
            consumer,
            PivotChoice::MaxDegreeLocal,
            candidates,
            Graph::VertexSet::new(),
            None,
        );
    }
}
