//! Core of Bron-Kerbosch algorithms with pivot

use super::graph::{UndirectedGraph, Vertex, VertexSetLike};
use super::pile::Pile;
use super::reporter::Reporter;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PivotChoice {
    Arbitrary,
    Random,
    MaxDegreeLocal,
    MaxDegreeLocalX,
}

type Clique<'a> = Pile<'a, Vertex>;

pub fn explore_with_pivot<VertexSet, Graph, Rprtr>(
    graph: &Graph,
    reporter: &mut Rprtr,
    pivot_selection: PivotChoice,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    let order = graph.order();
    if let Some(pivot) = (0..order).map(Vertex::new).max_by_key(|&v| graph.degree(v)) {
        let mut excluded = Graph::VertexSet::with_capacity(order);
        for v in (0..order).map(Vertex::new) {
            let neighbours = graph.neighbours(v);
            if !neighbours.is_empty() && !neighbours.contains(pivot) {
                let neighbouring_excluded: VertexSet = neighbours.intersection_collect(&excluded);
                if neighbouring_excluded.len() < neighbours.len() {
                    let neighbouring_candidates: VertexSet =
                        neighbours.difference_collect(&neighbouring_excluded);
                    visit(
                        graph,
                        reporter,
                        pivot_selection,
                        neighbouring_candidates,
                        neighbouring_excluded,
                        Some(&Pile::from(v)),
                    );
                }
                excluded.insert(v);
            }
        }
    }
}

pub fn visit<VertexSet, Graph, Rprtr>(
    graph: &Graph,
    reporter: &mut Rprtr,
    pivot_selection: PivotChoice,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Option<&Clique>,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));
    debug_assert!(candidates.len() >= 1);
    if candidates.len() == 1 {
        // Same logic as below, but stripped down for this common case
        candidates.for_each(|v| {
            let neighbours = graph.neighbours(v);
            if neighbours.is_disjoint(&excluded) {
                reporter.record(Pile::on(clique, v).collect());
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
                    if neighbours.is_disjoint(&excluded) {
                        reporter.record(Pile::on(clique, v).collect());
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
            let mut rng = rand::thread_rng();
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
                    reporter,
                    pivot_selection,
                    neighbouring_candidates,
                    neighbouring_excluded,
                    Some(&Pile::on(clique, v)),
                );
            } else if excluded.is_disjoint(neighbours) {
                reporter.record(Pile::on(clique, v).collect());
            }
            excluded.insert(v);
        }
    }
}
