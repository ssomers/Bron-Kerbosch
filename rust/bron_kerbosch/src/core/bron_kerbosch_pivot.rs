//! Core of Bron-Kerbosch algorithms with pivot

use super::clique_consumer::CliqueConsumer;
use super::graph::Graph;
use super::pile::Pile;
use super::vertex::{Vertex, VertexMap};
use super::vertexsetlike::VertexSetLike;
use std::ops::Not;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PivotChoice {
    Arbitrary,
    Random,
    MaxDegreeLocal,
    MaxDegreeLocalX,
}

type CliqueInProgress<'a> = Pile<'a, Vertex>;

pub fn explore_with_pivot<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    mut consumer: Consumer,
    pivot_selection: PivotChoice,
) -> Consumer::Harvest
where
    VertexSet: VertexSetLike,
    Consumer: CliqueConsumer,
{
    if let Some(pivot) = graph.max_degree_vertices().next() {
        let mut excluded = VertexMap::new(false, graph.order());
        for v in graph.vertices() {
            let neighbours = graph.neighbours(v);
            if neighbours.contains(pivot).not() {
                let neighbouring_excluded: VertexSet =
                    neighbours.filter_map(&excluded).copied().collect();
                if neighbouring_excluded.len() < neighbours.len() {
                    let neighbouring_candidates = neighbours
                        .difference(&neighbouring_excluded)
                        .copied()
                        .collect();
                    visit(
                        graph,
                        &mut consumer,
                        pivot_selection,
                        neighbouring_candidates,
                        neighbouring_excluded,
                        &Pile::from(v),
                    );
                }
                excluded[v] = true;
            }
        }
    }
    consumer.harvest()
}

pub fn visit<VertexSet, Consumer>(
    graph: &Graph<VertexSet>,
    consumer: &mut Consumer,
    pivot_selection: PivotChoice,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique_in_progress: &CliqueInProgress,
) where
    VertexSet: VertexSetLike,
    Consumer: CliqueConsumer,
{
    debug_assert!(candidates.all(|&v| graph.is_connected(v)));
    debug_assert!(excluded.all(|&v| graph.is_connected(v)));
    debug_assert!(candidates.is_disjoint(&excluded));
    debug_assert!(candidates.len() >= 1);
    if candidates.len() == 1 {
        // Same logic as below, but stripped down for this common case
        candidates.for_each(|v| {
            let neighbours = graph.neighbours(v);
            if consumer.is_accepted_size(clique_in_progress.height + 1)
                && neighbours.is_disjoint(&excluded)
            {
                consumer.accept(clique_in_progress.pile(v).iter().copied().collect());
            }
        });
        return;
    }

    let mut pivot: Option<Vertex> = None;
    let mut remaining_candidates: Vec<Vertex> = Vec::with_capacity(candidates.len());
    match pivot_selection {
        PivotChoice::MaxDegreeLocal | PivotChoice::MaxDegreeLocalX => {
            // Quickly handle locally unconnected candidates while finding pivot
            let mut seen_local_degree = 0;
            candidates.for_each(|v| {
                let neighbours = graph.neighbours(v);
                let local_degree = neighbours.intersection(&candidates).count();
                if local_degree == 0 {
                    // Same logic as below, but stripped down
                    if consumer.is_accepted_size(clique_in_progress.height + 1)
                        && neighbours.is_disjoint(&excluded)
                    {
                        consumer.accept(clique_in_progress.pile(v).iter().copied().collect());
                    }
                } else {
                    if seen_local_degree < local_degree {
                        seen_local_degree = local_degree;
                        pivot = Some(v);
                    }
                    remaining_candidates.push(v);
                }
            });
            if remaining_candidates.is_empty() {
                return;
            }
            if pivot_selection == PivotChoice::MaxDegreeLocalX {
                excluded.for_each(|v| {
                    let neighbours = graph.neighbours(v);
                    let local_degree = neighbours.intersection(&candidates).count();
                    if seen_local_degree < local_degree {
                        seen_local_degree = local_degree;
                        pivot = Some(v);
                    }
                });
            }
        }
        PivotChoice::Arbitrary => {
            candidates.for_each(|v| remaining_candidates.push(v));
            pivot = candidates.choose_arbitrary().copied();
        }
        PivotChoice::Random => {
            candidates.for_each(|v| remaining_candidates.push(v));
            let mut rng = rand::rng();
            pivot = candidates.choose(&mut rng).copied();
        }
    }

    debug_assert!(remaining_candidates.is_empty().not());
    let pivot = pivot.unwrap();
    for v in remaining_candidates {
        let neighbours = graph.neighbours(v);
        if neighbours.contains(pivot).not() {
            candidates.remove(v);
            let neighbouring_candidates: VertexSet =
                neighbours.intersection(&candidates).copied().collect();
            if neighbouring_candidates.is_empty().not() {
                let neighbouring_excluded = neighbours.intersection(&excluded).copied().collect();
                visit(
                    graph,
                    consumer,
                    pivot_selection,
                    neighbouring_candidates,
                    neighbouring_excluded,
                    &clique_in_progress.pile(v),
                );
            } else if consumer.is_accepted_size(clique_in_progress.height + 1)
                && excluded.is_disjoint(neighbours)
            {
                consumer.accept(clique_in_progress.pile(v).iter().copied().collect());
            }
            excluded.insert(v);
        }
    }
}
