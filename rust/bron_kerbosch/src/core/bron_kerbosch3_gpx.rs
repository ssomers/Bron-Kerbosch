//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

use super::bron_kerbosch_degen::{PivotChoice, explore_with_pivot};
use super::clique::CliqueConsumer;
use super::graph::{UndirectedGraph, VertexSetLike};

pub fn explore<VertexSet, Graph>(graph: &Graph, consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    explore_with_pivot(graph, consumer, PivotChoice::MaxDegreeLocalX)
}
