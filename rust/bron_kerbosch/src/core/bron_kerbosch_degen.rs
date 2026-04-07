//! Core of Bron-Kerbosch algorithms using degeneracy ordering.

pub use super::bron_kerbosch_pivot::PivotChoice;
use super::bron_kerbosch_pivot::visit;
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::graph_degeneracy::DegeneracyOrder;
use super::pile::Pile;
use super::vertexsetlike::VertexSetLike;

pub fn explore_with_pivot<VertexSet>(
    graph: &Graph<VertexSet>,
    mut consumer: CliqueConsumer,
    pivot_selection: PivotChoice,
) where
    VertexSet: VertexSetLike,
{
    DegeneracyOrder::on(graph).apply(|v, attorney| {
        let (neighbouring_candidates, neighbouring_excluded) = attorney.partition_neighbours(v);
        visit(
            graph,
            &mut consumer,
            pivot_selection,
            neighbouring_candidates,
            neighbouring_excluded,
            &Pile::from(v),
        );
    })
}
