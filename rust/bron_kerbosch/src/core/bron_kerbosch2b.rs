//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

use super::bron_kerbosch_pivot::{PivotChoice, explore_with_pivot};
use super::graph::{UndirectedGraph, VertexSetLike};
use super::reporter::Reporter;

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    explore_with_pivot(graph, reporter, PivotChoice::Arbitrary)
}
