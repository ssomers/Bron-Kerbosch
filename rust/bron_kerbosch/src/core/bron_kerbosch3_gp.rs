//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)

use super::bron_kerbosch_degen::{PivotChoice, explore_with_pivot};
use super::clique::CliqueConsumer;
use super::graphlike::{GraphLike, VertexSetLike};

pub fn explore<VertexSet, Graph>(graph: &Graph, consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
    Graph: GraphLike<VertexSet = VertexSet>,
{
    explore_with_pivot(graph, consumer, PivotChoice::MaxDegreeLocal)
}
