//! Bron-Kerbosch algorithm with pivot, slightly optimized and picking pivot randomly (IK_RP)

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_nodes, UndirectedGraph, VertexSetLike};
use pile::Pile;
use reporter::Reporter;

pub fn explore<VertexSet>(graph: &UndirectedGraph<VertexSet>, reporter: &mut Reporter)
where
    VertexSet: VertexSetLike,
{
    let candidates = connected_nodes(graph);
    if !candidates.is_empty() {
        visit(
            graph,
            reporter,
            PivotChoice::Random,
            PivotChoice::Random,
            candidates,
            VertexSet::new(),
            Pile::new(),
        );
    }
}
