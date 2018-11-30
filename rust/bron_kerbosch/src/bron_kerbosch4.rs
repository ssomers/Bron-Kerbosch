//! Bron-Kerbosch algorithm with pivot, slightly optimized and picking pivot randomly (IK_RP)
extern crate rand;

use self::rand::seq::IteratorRandom;
use graph::UndirectedGraph;
use graph::Vertex;
use reporter::Clique;
use reporter::Reporter;
use std::collections::HashSet;

pub fn explore(
    graph: &UndirectedGraph,
    clique: Clique,
    mut candidates: HashSet<Vertex>,
    mut excluded: HashSet<Vertex>,
    reporter: &mut Reporter,
) {
    debug_assert!(candidates.iter().all(|v| graph.degree(*v) > 0));
    debug_assert!(excluded.iter().all(|v| graph.degree(*v) > 0));
    reporter.inc_count();
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(&clique);
        return;
    }

    let pivot = pick_random(if !candidates.is_empty() {
        &candidates
    } else {
        &excluded
    });
    let far_candidates: Vec<Vertex> = candidates
        .difference(graph.adjacencies(pivot))
        .cloned()
        .collect();
    excluded.reserve(far_candidates.len());
    for v in far_candidates {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        candidates.remove(&v);
        let neighbouring_candidates: HashSet<Vertex> =
            neighbours.intersection(&candidates).cloned().collect();
        let neighbouring_excluded: HashSet<Vertex> =
            neighbours.intersection(&excluded).cloned().collect();
        excluded.insert(v);
        explore(
            graph,
            [clique.as_slice(), &[v]].concat(),
            neighbouring_candidates,
            neighbouring_excluded,
            reporter,
        );
    }
}

fn pick_random(s: &HashSet<Vertex>) -> Vertex {
    let mut rng = rand::thread_rng();
    s.iter().choose(&mut rng).unwrap().clone()
}
