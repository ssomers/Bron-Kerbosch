//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)
//! implemented by multiple threads

use super::bron_kerbosch_degen_mt::{PivotChoice, explore_with_pivot_multithreaded};
use super::graph::{UndirectedGraph, VertexSetLike};
use super::reporter::Reporter;

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    const NUM_VISITING_THREADS: usize = 5;
    explore_with_pivot_multithreaded(
        graph,
        reporter,
        PivotChoice::MaxDegreeLocal,
        NUM_VISITING_THREADS,
    )
}
