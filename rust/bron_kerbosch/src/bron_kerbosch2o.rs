//! Bron-Kerbosch algorithm with pivot picked arbitrarily, slightly optimized

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_nodes, UndirectedGraph, VertexSet};
use pile::Pile;
use reporter::Reporter;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
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
