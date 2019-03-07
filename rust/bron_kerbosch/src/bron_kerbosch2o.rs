//! Bron-Kerbosch algorithm with pivot picked arbitrarily, slightly optimized

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_nodes, UndirectedGraph, VertexSetLike};
use pile::Pile;
use reporter::Reporter;

pub fn explore<VertexSet>(graph: &UndirectedGraph<VertexSet>, reporter: &mut Reporter)
where
    VertexSet: VertexSetLike<VertexSet>,
{
    let candidates = connected_nodes(graph);
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            reporter,
            PivotChoice::Arbitrary,
            PivotChoice::Arbitrary,
            candidates,
            VertexSet::with_capacity(num_candidates),
            Pile::new(),
        );
    }
}
