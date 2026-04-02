//! Core of Bron-Kerbosch algorithms with pivot

use super::clique::CliqueConsumer;
use super::graph::max_degree_vertices;
use super::graph::{UndirectedGraph, VertexSetLike, vertices};
use super::pile::Pile;
use super::vertex::{Vertex, VertexMap};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PivotChoice {
    Arbitrary,
    Random,
    MaxDegreeLocal,
    MaxDegreeLocalX,
}

type CliqueInProgress<'a> = Pile<'a, Vertex>;

pub fn explore_with_pivot<VertexSet, Graph>(
    graph: &Graph,
    mut consumer: CliqueConsumer,
    pivot_selection: PivotChoice,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    if let Some(pivot) = max_degree_vertices(graph).next() {
        let mut excluded = VertexMap::new(false, graph.order());
        for v in vertices(graph) {
            let neighbours = graph.neighbours(v);
            if !neighbours.is_empty() && !neighbours.contains(pivot) {
                let neighbouring_excluded: VertexSet =
                    neighbours.intersection_with_fn_collect(|v| excluded[v]);
                if neighbouring_excluded.len() < neighbours.len() {
                    let neighbouring_candidates: VertexSet =
                        neighbours.difference_collect(&neighbouring_excluded);
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
}

pub fn visit<VertexSet, Graph>(
    graph: &Graph,
    consumer: &mut CliqueConsumer,
    pivot_selection: PivotChoice,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique_in_progress: &CliqueInProgress,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));
    debug_assert!(candidates.len() >= 1);
    if candidates.len() == 1 {
        // Same logic as below, but stripped down for this common case
        candidates.for_each(|v| {
            let neighbours = graph.neighbours(v);
            if consumer.is_accepted_size(clique_in_progress.height + 1)
                && neighbours.is_disjoint(&excluded)
            {
                consumer.accept(clique_in_progress.pile(v).collect());
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
                let local_degree = neighbours.intersection_size(&candidates);
                if local_degree == 0 {
                    // Same logic as below, but stripped down
                    if consumer.is_accepted_size(clique_in_progress.height + 1)
                        && neighbours.is_disjoint(&excluded)
                    {
                        consumer.accept(clique_in_progress.pile(v).collect());
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
                    let local_degree = neighbours.intersection_size(&candidates);
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

    debug_assert!(!remaining_candidates.is_empty());
    let pivot = pivot.unwrap();
    for v in remaining_candidates {
        let neighbours = graph.neighbours(v);
        if !neighbours.contains(pivot) {
            candidates.remove(v);
            let neighbouring_candidates: VertexSet = neighbours.intersection_collect(&candidates);
            if !neighbouring_candidates.is_empty() {
                let neighbouring_excluded = neighbours.intersection_collect(&excluded);
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
                consumer.accept(clique_in_progress.pile(v).collect());
            }
            excluded.insert(v);
        }
    }
}
