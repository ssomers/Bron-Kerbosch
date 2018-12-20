//! Bron-Kerbosch algorithm with pivot, slightly optimized and picking pivot
//! with highest degree (IK_GP)

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::UndirectedGraph;
use reporter::Reporter;
use vertex_stack::VertexStack;

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates = graph.connected_nodes();
    if !candidates.is_empty() {
        visit(
            graph,
            reporter,
            PivotChoice::MaxDegree,
            PivotChoice::MaxDegree,
            candidates,
            HashSet::new(),
            VertexStack::Empty,
        );
    }
}
