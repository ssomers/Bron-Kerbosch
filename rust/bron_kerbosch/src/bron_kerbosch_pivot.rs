//! Core of Bron-Kerbosch algorithms with pivot

use graph::{UndirectedGraph, Vertex, VertexSetLike};
use pile::Pile;
use reporter::Reporter;

#[derive(Clone, Debug)]
pub enum PivotChoice {
    Arbitrary,
    Random,
    MaxDegree,
    MaxDegreeLocal,
}

type Clique<'a> = Pile<'a, Vertex>;

pub fn visit<VertexSet>(
    graph: &UndirectedGraph<VertexSet>,
    reporter: &mut Reporter,
    initial_pivot_selection: PivotChoice,
    further_pivot_selection: PivotChoice,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Clique,
) where
    VertexSet: VertexSetLike,
{
    debug_assert!(!candidates.is_empty());
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));

    if candidates.len() == 1 {
        candidates.for_each(|v| {
            let neighbours = graph.neighbours(v);
            if neighbours.is_disjoint(&excluded) {
                reporter.record(clique.place(v).collect());
            }
        });
        return;
    }

    let &pivot = choose(initial_pivot_selection, &candidates, &excluded, graph).unwrap();
    let far_candidates: Vec<Vertex> = candidates.difference(graph.neighbours(pivot));
    excluded.reserve(far_candidates.len());
    for v in far_candidates {
        let neighbours = graph.neighbours(v);
        candidates.remove(&v);
        let neighbouring_candidates: VertexSet = neighbours.intersection(&candidates);
        if !neighbouring_candidates.is_empty() {
            let neighbouring_excluded: VertexSet = neighbours.intersection(&excluded);
            visit(
                graph,
                reporter,
                further_pivot_selection.clone(),
                further_pivot_selection.clone(),
                neighbouring_candidates,
                neighbouring_excluded,
                clique.place(v),
            );
        } else {
            if neighbours.is_disjoint(&excluded) {
                reporter.record(clique.place(v).collect());
            }
        }
        excluded.insert(v);
    }
}

fn choose<'a, VertexSet>(
    pivot_choice: PivotChoice,
    candidates: &'a VertexSet,
    excluded: &'a VertexSet,
    graph: &UndirectedGraph<VertexSet>,
) -> Option<&'a Vertex>
where
    VertexSet: VertexSetLike,
{
    match pivot_choice {
        PivotChoice::Arbitrary => candidates.choose_arbitrary(),
        PivotChoice::Random => {
            let mut rng = rand::thread_rng();
            candidates.choose(&mut rng)
        }
        PivotChoice::MaxDegree => candidates.max_by_key(|&&v| graph.degree(v) as usize),
        PivotChoice::MaxDegreeLocal => candidates.max_by_key_from_either(excluded, |&&v| {
            graph.neighbours(v).intersection_size(&candidates)
        }),
    }
}
