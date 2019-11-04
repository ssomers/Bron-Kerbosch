//! Bron-Kerbosch algorithm with pivot picked arbitrarily

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_vertices, UndirectedGraph, VertexSetLike};
use pile::Pile;
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
            PivotChoice::Arbitrary,
            PivotChoice::Arbitrary,
            candidates,
            Graph::VertexSet::new(),
            Pile::new(),
        );
    }
}
