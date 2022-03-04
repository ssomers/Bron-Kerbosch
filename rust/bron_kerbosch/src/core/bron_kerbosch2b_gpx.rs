//! Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

use super::bron_kerbosch_pivot::{explore_with_pivot, PivotChoice};
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
