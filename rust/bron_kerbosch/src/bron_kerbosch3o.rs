//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use bron_kerbosch_degeneracy::degeneracy_order_smart;
use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{connected_nodes, UndirectedGraph, VertexSetLike};
use pile::Pile;
use reporter::Reporter;

pub fn explore<VertexSet>(graph: &UndirectedGraph<VertexSet>, reporter: &mut Reporter)
where
    VertexSet: VertexSetLike,
{
    let mut candidates = connected_nodes(graph);
    debug_assert_eq!(
        candidates.len(),
        degeneracy_order_smart(graph, &candidates).count()
    );
    debug_assert_eq!(
        candidates,
        degeneracy_order_smart(graph, &candidates)
            .into_iter()
            .collect()
    );
    let mut excluded = VertexSet::with_capacity(candidates.len());
    for v in degeneracy_order_smart(graph, &candidates) {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        candidates.remove(&v);
        let neighbouring_candidates = neighbours.intersection(&candidates);
        let neighbouring_excluded = neighbours.intersection(&excluded);
        excluded.insert(v);
        visit(
            graph,
            reporter,
            PivotChoice::MaxDegree,
            PivotChoice::MaxDegree,
            neighbouring_candidates,
            neighbouring_excluded,
            Pile::from(v),
        );
    }
}
