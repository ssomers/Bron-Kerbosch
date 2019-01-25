//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use bron_kerbosch_degeneracy::degeneracy_order_smart;
use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_nodes, UndirectedGraph};
use pile::Pile;
use reporter::Reporter;

use std::collections::HashSet;

pub fn explore(graph: &impl UndirectedGraph, reporter: &mut Reporter) {
    let mut candidates = connected_nodes(graph);
    debug_assert_eq!(
        degeneracy_order_smart(graph, &candidates).collect::<HashSet<_>>(),
        candidates
    );
    let mut excluded = HashSet::with_capacity(candidates.len());
    for v in degeneracy_order_smart(graph, &candidates) {
        candidates.remove(&v);
        let neighbouring_candidates = graph.neighbour_intersection(&candidates, v);
        let neighbouring_excluded = graph.neighbour_intersection(&excluded, v);
        excluded.insert(v);
        visit(
            graph,
            reporter,
            PivotChoice::MaxDegree,
            PivotChoice::MaxDegree,
            neighbouring_candidates,
            neighbouring_excluded,
            Pile::new().cons(v),
        );
    }
}
