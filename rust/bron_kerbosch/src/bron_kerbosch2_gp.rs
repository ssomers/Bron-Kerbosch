//! Bron-Kerbosch algorithm with pivot, slightly optimized and picking pivot
//! with highest degree (IK_GP)

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_nodes, UndirectedGraph};
use pile::Pile;
use reporter::Reporter;

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates = connected_nodes(graph);
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            reporter,
            PivotChoice::MaxDegree,
            PivotChoice::MaxDegree,
            candidates,
            HashSet::with_capacity(num_candidates),
            Pile::new(),
        );
    }
}
