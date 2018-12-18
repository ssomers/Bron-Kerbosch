//! Core of Bron-Kerbosch algorithms with pivot

use graph::{UndirectedGraph, Vertex};
use reporter::Reporter;
use vertex_stack::VertexStack;

extern crate rand;
use self::rand::seq::IteratorRandom;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum PivotChoice {
    Random,
    MaxDegree,
    MaxDegreeLocal,
}

pub fn visit(
    graph: &UndirectedGraph,
    reporter: &mut Reporter,
    initial_pivot_selection: PivotChoice,
    further_pivot_selection: PivotChoice,
    mut candidates: HashSet<Vertex>,
    mut excluded: HashSet<Vertex>,
    clique: VertexStack,
) {
    debug_assert!(candidates.iter().all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.iter().all(|&v| graph.degree(v) > 0));
    reporter.inc_count();
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(clique.collect());
        return;
    }

    let pivot = match initial_pivot_selection {
        PivotChoice::Random => pick_random(&candidates, &excluded),
        PivotChoice::MaxDegree => pick_max_degree(graph, &candidates, &excluded),
        PivotChoice::MaxDegreeLocal => pick_max_degree_local(graph, &candidates, &excluded),
    };
    let far_candidates: Vec<Vertex> = candidates
        .difference(graph.adjacencies(pivot))
        .cloned()
        .collect();
    excluded.reserve(far_candidates.len());
    for v in far_candidates {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        candidates.remove(&v);
        let neighbouring_candidates = neighbours.intersection(&candidates).cloned().collect();
        let neighbouring_excluded = neighbours.intersection(&excluded).cloned().collect();
        excluded.insert(v);
        visit(
            graph,
            reporter,
            further_pivot_selection.clone(),
            further_pivot_selection.clone(),
            neighbouring_candidates,
            neighbouring_excluded,
            VertexStack::Cons(&clique, v),
        );
    }
}

fn smart_intersect<'a, T, S>(
    s1: &'a HashSet<T, S>,
    s2: &'a HashSet<T, S>,
) -> std::collections::hash_set::Intersection<'a, T, S>
where
    T: Eq + std::hash::Hash,
    S: std::hash::BuildHasher,
{
    if s1.len() < s2.len() {
        s1.intersection(s2)
    } else {
        s2.intersection(s1)
    }
}

fn pick_random(candidates: &HashSet<Vertex>, excluded: &HashSet<Vertex>) -> Vertex {
    let mut rng = rand::thread_rng();
    let s = if !candidates.is_empty() {
        &candidates
    } else {
        &excluded
    };
    *s.iter().choose(&mut rng).unwrap()
}

fn max_degree(graph: &UndirectedGraph, vertices: impl Iterator<Item = Vertex>) -> Vertex {
    vertices.max_by_key(|&v| graph.degree(v)).unwrap()
}

fn pick_max_degree(
    graph: &UndirectedGraph,
    candidates: &HashSet<Vertex>,
    excluded: &HashSet<Vertex>,
) -> Vertex {
    debug_assert!(!(candidates.is_empty() && excluded.is_empty()));
    max_degree(graph, candidates.iter().chain(excluded).cloned())
}

fn pick_max_degree_local(
    graph: &UndirectedGraph,
    candidates: &HashSet<Vertex>,
    excluded: &HashSet<Vertex>,
) -> Vertex {
    debug_assert!(!(candidates.is_empty() && excluded.is_empty()));
    candidates
        .iter()
        .chain(excluded)
        .cloned()
        .max_by_key(|&v| smart_intersect(graph.adjacencies(v), &candidates).count())
        .unwrap()
}
