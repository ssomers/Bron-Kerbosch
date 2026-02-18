//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)

use super::base::CliqueConsumer;
use super::bron_kerbosch_degen::{PivotChoice, explore_with_pivot};
use super::graph::{UndirectedGraph, VertexSetLike};

pub fn explore<VertexSet, Graph, Consumer>(graph: &Graph, consumer: &mut Consumer)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Consumer: CliqueConsumer,
{
    explore_with_pivot(graph, consumer, PivotChoice::MaxDegreeLocal)
}
