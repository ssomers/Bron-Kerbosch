//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

use crate::bron_kerbosch_pivot::{visit, PivotChoice};
use crate::graph::{UndirectedGraph, VertexSetLike};
use crate::graph_degeneracy::degeneracy_ordering;
use crate::pile::Pile;
use crate::reporter::Reporter;

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
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
                PivotChoice::MaxDegreeLocalX,
                PivotChoice::MaxDegreeLocalX,
                neighbouring_candidates,
                neighbouring_excluded,
                Some(&Pile::from(v)),
            );
        }
        excluded.insert(v);
    }
}
