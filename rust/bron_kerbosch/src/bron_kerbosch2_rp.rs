//! Bron-Kerbosch algorithm with pivot, slightly optimized and picking pivot randomly (IK_RP)

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
            PivotChoice::Random,
            PivotChoice::Random,
            candidates,
            VertexSet::with_capacity(num_candidates),
            Pile::new(),
        );
    }
}
