//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use crate::bron_kerbosch_pivot::{visit, PivotChoice};
use crate::graph::{connected_vertices, UndirectedGraph, VertexSetLike};
use crate::reporter::Reporter;

pub fn explore<Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    Graph: UndirectedGraph,
    Rprtr: Reporter,
{
    let candidates = connected_vertices(graph);
    if !candidates.is_empty() {
        visit(
            graph,
            reporter,
            PivotChoice::MaxDegree,
            PivotChoice::MaxDegreeLocal,
            candidates,
            Graph::VertexSet::new(),
            None,
        );
    }
}
