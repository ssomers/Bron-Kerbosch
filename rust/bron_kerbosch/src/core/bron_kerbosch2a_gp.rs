//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::base::CliqueConsumer;
use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::graph::{UndirectedGraph, VertexSetLike, connected_vertices};

pub fn explore<Graph, Consumer>(graph: &Graph, consumer: &mut Consumer)
where
    Graph: UndirectedGraph,
    Consumer: CliqueConsumer,
{
    let candidates = connected_vertices(graph);
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
