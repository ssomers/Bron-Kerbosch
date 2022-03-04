//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

use super::bron_kerbosch_degen::{explore_with_pivot, PivotChoice};
use super::graph::{UndirectedGraph, VertexSetLike};
use super::reporter::Reporter;

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    explore_with_pivot(graph, reporter, PivotChoice::MaxDegreeLocalX)
}
