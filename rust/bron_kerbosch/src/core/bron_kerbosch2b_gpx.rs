//! Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

use super::base::CliqueConsumer;
use super::bron_kerbosch_pivot::{PivotChoice, explore_with_pivot};
use super::graph::{UndirectedGraph, VertexSetLike};

pub fn explore<VertexSet, Graph, Consumer>(graph: &Graph, consumer: &mut Consumer)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Consumer: CliqueConsumer,
{
    explore_with_pivot(graph, consumer, PivotChoice::MaxDegreeLocalX)
}
