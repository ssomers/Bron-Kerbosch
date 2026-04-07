//! Core of Bron-Kerbosch algorithms using degeneracy ordering.

pub use super::bron_kerbosch_pivot::PivotChoice;
use super::bron_kerbosch_pivot::visit;
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::graph_degeneracy::DegeneracyOrder;
use super::pile::Pile;
use super::vertexsetlike::VertexSetLike;
use std::ops::Not;

pub fn explore_with_pivot<VertexSet>(
    graph: &Graph<VertexSet>,
    mut consumer: CliqueConsumer,
    pivot_selection: PivotChoice,
) where
    VertexSet: VertexSetLike,
{
    DegeneracyOrder::on(graph).apply(|v, vertex_info| {
        let mut neighbouring_candidates = VertexSet::new();
        let mut neighbouring_excluded = VertexSet::new();
        graph.neighbours(v).for_each(|v| {
            if vertex_info.is_candidate(v) {
                neighbouring_candidates.insert(v);
            } else {
                neighbouring_excluded.insert(v);
            }
        });
        debug_assert!(neighbouring_candidates.is_empty().not());
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
