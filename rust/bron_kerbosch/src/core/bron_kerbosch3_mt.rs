//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)
//! implemented by multiple threads

use super::bron_kerbosch_degen_mt::{PivotChoice, explore_with_pivot_multithreaded};
use super::clique::CliqueConsumer;
use super::graphlike::{GraphLike, VertexSetLike};

pub fn explore<VertexSet, Graph>(graph: &Graph, consumer: CliqueConsumer)
where
    VertexSet: VertexSetLike,
    Graph: GraphLike<VertexSet = VertexSet>,
{
    const NUM_VISITING_THREADS: usize = 5;
    explore_with_pivot_multithreaded(
        graph,
        consumer,
        PivotChoice::MaxDegreeLocal,
        NUM_VISITING_THREADS,
    )
}
