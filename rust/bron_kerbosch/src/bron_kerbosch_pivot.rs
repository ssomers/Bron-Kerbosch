//! Core of Bron-Kerbosch algorithms with pivot

use graph::{vertex_set_reserve, UndirectedGraph, Vertex, VertexSet};
use pile::Pile;
use reporter::Reporter;
use util::intersect;

extern crate rand;
use self::rand::seq::IteratorRandom;

#[derive(Clone, Debug)]
pub enum PivotChoice {
    Arbitrary,
    Random,
    MaxDegree,
    MaxDegreeLocal,
}

type Clique<'a> = Pile<'a, Vertex>;

pub fn visit(
    graph: &UndirectedGraph,
    reporter: &mut Reporter,
    initial_pivot_selection: PivotChoice,
    further_pivot_selection: PivotChoice,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Clique,
) {
    debug_assert!(candidates.iter().all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.iter().all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));

    if candidates.is_empty() {
        if excluded.is_empty() {
            reporter.record(clique.collect());
        }
        return;
    }

    let pivot = match initial_pivot_selection {
        PivotChoice::Arbitrary => pick_arbitrary(&candidates, &excluded),
        PivotChoice::Random => pick_random(&candidates, &excluded),
        PivotChoice::MaxDegree => pick_max_degree(graph, &candidates, &excluded),
        PivotChoice::MaxDegreeLocal => pick_max_degree_local(graph, &candidates, &excluded),
    };
    let far_candidates: Vec<Vertex> = candidates
        .difference(graph.neighbours(pivot))
        .cloned()
        .collect();
    vertex_set_reserve(&mut excluded, far_candidates.len());
    for v in far_candidates {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        candidates.remove(&v);
        let neighbouring_candidates = intersect(&neighbours, &candidates).cloned().collect();
        let neighbouring_excluded = intersect(&neighbours, &excluded).cloned().collect();
        excluded.insert(v);
        visit(
            graph,
            reporter,
            further_pivot_selection.clone(),
            further_pivot_selection.clone(),
            neighbouring_candidates,
            neighbouring_excluded,
            clique.cons(v),
        );
    }
}

fn pick_arbitrary(candidates: &VertexSet, _excluded: &VertexSet) -> Vertex {
    debug_assert!(!candidates.is_empty());
    *candidates.iter().next().unwrap()
}

fn pick_random(candidates: &VertexSet, _excluded: &VertexSet) -> Vertex {
    debug_assert!(!candidates.is_empty());
    let mut rng = rand::thread_rng();
    *candidates.iter().choose(&mut rng).unwrap()
}

fn pick_max_degree(
    graph: &UndirectedGraph,
    candidates: &VertexSet,
    excluded: &VertexSet,
) -> Vertex {
    debug_assert!(!candidates.is_empty());
    candidates
        .iter()
        .chain(excluded)
        .cloned()
        .max_by_key(|&v| graph.degree(v))
        .unwrap()
}

fn pick_max_degree_local(
    graph: &UndirectedGraph,
    candidates: &VertexSet,
    excluded: &VertexSet,
) -> Vertex {
    debug_assert!(!candidates.is_empty());
    candidates
        .iter()
        .chain(excluded)
        .cloned()
        .max_by_key(|&v| intersect(&graph.neighbours(v), &candidates).count())
        .unwrap()
}
