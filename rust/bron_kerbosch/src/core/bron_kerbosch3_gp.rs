//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)

use super::bron_kerbosch_pivot::{visit, PivotChoice};
use super::graph::{UndirectedGraph, VertexSetLike};
use super::graph_degeneracy::degeneracy_ordering;
use super::pile::Pile;
use super::reporter::Reporter;

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    // In this initial iteration, we don't need to represent the set of candidates
    // because all neighbours are candidates until excluded.
    let mut excluded = VertexSet::with_capacity(graph.order());
    for v in degeneracy_ordering(graph, -1) {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_excluded: VertexSet = neighbours.intersection_collect(&excluded);
        if neighbouring_excluded.len() < neighbours.len() {
            let neighbouring_candidates: VertexSet =
                neighbours.difference_collect(&neighbouring_excluded);
            visit(
                graph,
                reporter,
                PivotChoice::MaxDegreeLocal,
                neighbouring_candidates,
                neighbouring_excluded,
                Some(&Pile::from(v)),
            );
        }
        excluded.insert(v);
    }
}
