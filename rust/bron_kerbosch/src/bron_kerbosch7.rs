//! Bron-Kerbosch algorithm with pivot, slightly optimized and picking pivot
//! with highest degree towards the remaining candidates (IK_GPX)

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::UndirectedGraph;
use pile::Pile;
use reporter::Reporter;

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates = graph.connected_nodes();
    if !candidates.is_empty() {
        visit(
            graph,
            reporter,
            PivotChoice::MaxDegree,
            PivotChoice::MaxDegreeLocal,
            candidates,
            HashSet::new(),
            Pile::Empty,
        );
    }
}
