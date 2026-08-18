//! Core of Bron-Kerbosch algorithms using degeneracy ordering.

use super::bron_kerbosch_pivot::{PivotChoice, visit};
use super::clique_consumer::CliqueConsumer;
use super::degeneracy::Degeneracy;
use super::pile::Pile;
use crate::{CliqueAccumulator, Graph, VertexSetLike};

pub fn explore_with_degeneracy<VertexSet, Accumulator>(
    graph: &Graph<VertexSet>,
    min_clique_size: usize,
    mut accumulator: Accumulator,
    pivot_selection: PivotChoice,
) -> Accumulator::Harvest
where
    VertexSet: VertexSetLike,
    Accumulator: CliqueAccumulator,
{
    let mut consumer = CliqueConsumer {
        min_clique_size,
        accu: &mut accumulator,
    };
    Degeneracy::on(graph).apply(|v, attorney| {
        let (neighbouring_candidates, neighbouring_excluded) = attorney.partition_neighbours(v);
        visit(
            graph,
            &mut consumer,
            pivot_selection,
            neighbouring_candidates,
            neighbouring_excluded,
            &Pile::from(v),
        );
    });
    accumulator.harvest()
}
