//! Core of Bron-Kerbosch algorithms using degeneracy ordering.

pub use super::bron_kerbosch_pivot::PivotChoice;
use super::bron_kerbosch_pivot::visit;
use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::graph_degeneracy::degeneracy_iter;
use super::pile::Pile;
use super::vertexsetlike::VertexSetLike;

pub fn explore_with_pivot<VertexSet>(
    graph: &Graph<VertexSet>,
    mut consumer: CliqueConsumer,
    pivot_selection: PivotChoice,
) where
    VertexSet: VertexSetLike,
{
    // In this initial iteration, we don't need to represent the set of candidates
    // because all neighbours are candidates until excluded.
    for (v, neighbouring_excluded) in degeneracy_iter(graph) {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        debug_assert!(neighbouring_excluded.len() < neighbours.len());
        let neighbouring_candidates = neighbours.difference_collect(&neighbouring_excluded);
        visit(
            graph,
            &mut consumer,
            pivot_selection,
            neighbouring_candidates,
            neighbouring_excluded,
            &Pile::from(v),
        );
    }
}
