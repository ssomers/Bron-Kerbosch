//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)

use super::bron_kerbosch_degen::{explore_with_pivot, PivotChoice};
use super::graph::{UndirectedGraph, VertexSetLike};
use super::reporter::Reporter;

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    explore_with_pivot(graph, reporter, PivotChoice::MaxDegreeLocal)
}
