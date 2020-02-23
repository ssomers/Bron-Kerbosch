//! Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_vertices, UndirectedGraph, VertexSetLike};
use reporter::Reporter;

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
            PivotChoice::MaxDegreeLocalX,
            candidates,
            Graph::VertexSet::new(),
            None,
        );
    }
}
