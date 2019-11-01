//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{UndirectedGraph, VertexSetLike};
use graph_degeneracy::degeneracy_ordering;
use pile::Pile;
use reporter::Reporter;

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet>,
    Rprtr: Reporter,
{
    let mut excluded = VertexSet::with_capacity((graph.order() as usize).saturating_sub(1));
    for v in degeneracy_ordering(graph, -1) {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates: VertexSet = neighbours.difference_collect(&excluded);
        if neighbouring_candidates.is_empty() {
            debug_assert!(!neighbours.is_disjoint(&excluded));
        } else {
            let neighbouring_excluded: VertexSet = neighbours.intersection_collect(&excluded);
            visit(
                graph,
                reporter,
                PivotChoice::MaxDegreeLocal,
                PivotChoice::MaxDegreeLocal,
                neighbouring_candidates,
                neighbouring_excluded,
                Pile::from(v),
            );
        }
        excluded.insert(v);
    }
}
