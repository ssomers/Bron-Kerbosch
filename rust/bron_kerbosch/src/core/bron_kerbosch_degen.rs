//! Core of Bron-Kerbosch algorithms using degeneracy ordering.

pub use super::bron_kerbosch_pivot::PivotChoice;
use super::bron_kerbosch_pivot::visit;
use super::clique_consumer::CliqueConsumer;
use super::degeneracy::Degeneracy;
use super::graph::Graph;
use super::pile::Pile;
use super::vertexsetlike::VertexSetLike;

pub fn explore_with_degeneracy<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    mut consumer: Consumer,
    pivot_selection: PivotChoice,
) -> Consumer::Harvest
where
    VertexSet: VertexSetLike,
    Consumer: CliqueConsumer,
{
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
    consumer.harvest()
}
