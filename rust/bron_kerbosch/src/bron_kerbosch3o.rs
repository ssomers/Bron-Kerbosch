//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use bron_kerbosch_degeneracy::degeneracy_order_smart;
use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_nodes, UndirectedGraph};
use pile::Pile;
use reporter::Reporter;
use util::intersect;

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let mut candidates = connected_nodes(graph);
    debug_assert_eq!(
        degeneracy_order_smart(graph, &candidates).collect::<HashSet<_>>(),
        candidates
    );
    let mut excluded = HashSet::with_capacity(candidates.len());
    for v in degeneracy_order_smart(graph, &candidates) {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        candidates.remove(&v);
        let neighbouring_candidates = intersect(&neighbours, &candidates).cloned().collect();
        let neighbouring_excluded = intersect(&neighbours, &excluded).cloned().collect();
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
