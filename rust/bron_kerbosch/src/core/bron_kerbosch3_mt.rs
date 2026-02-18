//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)
//! implemented by multiple threads

use super::base::CliqueConsumer;
use super::bron_kerbosch_degen_mt::{PivotChoice, explore_with_pivot_multithreaded};
use super::graph::{UndirectedGraph, VertexSetLike};

pub fn explore<VertexSet, Graph, Consumer>(graph: &Graph, consumer: &mut Consumer)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Consumer: CliqueConsumer,
{
    const NUM_VISITING_THREADS: usize = 5;
    explore_with_pivot_multithreaded(
        graph,
        consumer,
        PivotChoice::MaxDegreeLocal,
        NUM_VISITING_THREADS,
    )
}
