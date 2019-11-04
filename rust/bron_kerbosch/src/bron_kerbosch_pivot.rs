//! Core of Bron-Kerbosch algorithms with pivot

use graph::{UndirectedGraph, Vertex, VertexSetLike};
use pile::Pile;
use reporter::Reporter;

use graph::rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PivotChoice {
    Arbitrary,
    Random,
    MaxDegree,
    MaxDegreeLocal,
    MaxDegreeLocalX,
}

type Clique<'a> = Pile<'a, Vertex>;

pub fn visit<VertexSet, Graph, Rprtr>(
    graph: &Graph,
    reporter: &mut Rprtr,
    initial_pivot_selection: PivotChoice,
    further_pivot_selection: PivotChoice,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Clique,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    debug_assert!(!candidates.is_empty());
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));

    if candidates.len() == 1 {
        // Same logic as below, but stripped down for this common case
        candidates.for_each(|v| {
            let neighbours = graph.neighbours(v);
            if neighbours.is_disjoint(&excluded) {
                reporter.record(clique.place(v).collect());
            }
        });
        return;
    }

    let mut pivot: Option<Vertex> = None;
    let mut remaining_candidates: Vec<Vertex> = Vec::with_capacity(candidates.len());
    match initial_pivot_selection {
        PivotChoice::MaxDegreeLocal | PivotChoice::MaxDegreeLocalX => {
            // Quickly handle locally unconnected candidates while finding pivot
            let mut seen_local_degree = 0;
            candidates.for_each(|v| {
                let neighbours = graph.neighbours(v);
                let local_degree = neighbours.intersection_size(&candidates);
                if local_degree == 0 {
                    // Same logic as below, but stripped down
                    if neighbours.is_disjoint(&excluded) {
                        reporter.record(clique.place(v).collect());
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
            if initial_pivot_selection == PivotChoice::MaxDegreeLocalX {
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
        _ => {
            candidates.for_each(|v| remaining_candidates.push(v));
            pivot = choose(initial_pivot_selection, &remaining_candidates, graph).copied();
        }
    }

    debug_assert!(!remaining_candidates.is_empty());
    let pivot = pivot.unwrap();
    for v in remaining_candidates {
        let neighbours = graph.neighbours(v);
        if neighbours.contains(pivot) {
            continue;
        }
        candidates.remove(v);
        let neighbouring_candidates: VertexSet = neighbours.intersection_collect(&candidates);
        if neighbouring_candidates.is_empty() {
            if neighbours.is_disjoint(&excluded) {
                reporter.record(clique.place(v).collect());
            }
        } else {
            let neighbouring_excluded: VertexSet = neighbours.intersection_collect(&excluded);
            visit(
                graph,
                reporter,
                further_pivot_selection,
                further_pivot_selection,
                neighbouring_candidates,
                neighbouring_excluded,
                clique.place(v),
            );
        }
        excluded.insert(v);
    }
}

fn choose<'a, Graph>(
    pivot_choice: PivotChoice,
    candidates: &'a [Vertex],
    graph: &Graph,
) -> Option<&'a Vertex>
where
    Graph: UndirectedGraph,
{
    match pivot_choice {
        PivotChoice::Arbitrary => candidates.first(),
        PivotChoice::Random => {
            let mut rng = rand::thread_rng();
            candidates.choose(&mut rng)
        }
        PivotChoice::MaxDegree => candidates.iter().max_by_key(|&&v| graph.degree(v) as usize),
        _ => panic!("Implemented separately"),
    }
}
