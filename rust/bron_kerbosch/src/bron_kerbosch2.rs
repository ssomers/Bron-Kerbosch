//! Bron-Kerbosch algorithm with pivot
extern crate rand;

use graph::UndirectedGraph;
use graph::Vertex;
use reporter::Clique;
use reporter::Reporter;
use std::collections::HashSet;

pub fn explore(
    graph: &UndirectedGraph,
    clique: Clique,
    candidates: &mut HashSet<Vertex>,
    excluded: &mut HashSet<Vertex>,
    reporter: &mut Reporter,
) {
    debug_assert!(candidates.iter().all(|v| graph.degree(*v) > 0));
    debug_assert!(excluded.iter().all(|v| graph.degree(*v) > 0));
    reporter.inc_count();
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(&clique);
        return;
    }

    let pivot = pick_arbitrary(if !candidates.is_empty() {
        &candidates
    } else {
        &excluded
    });
    debug_assert!(graph.degree(pivot) > 0);
    let far_candidates: HashSet<Vertex> = candidates
        .difference(graph.adjacencies(pivot))
        .cloned()
        .collect();
    for v in far_candidates {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        let mut nearby_candidates: HashSet<Vertex> =
            candidates.intersection(&neighbours).cloned().collect();
        let mut nearby_excluded: HashSet<Vertex> =
            excluded.intersection(&neighbours).cloned().collect();
        explore(
            graph,
            [clique.as_slice(), &[v]].concat(),
            &mut nearby_candidates,
            &mut nearby_excluded,
            reporter,
        );
        candidates.remove(&v);
        excluded.insert(v);
    }
}

fn pick_arbitrary(s: &HashSet<Vertex>) -> Vertex {
    s.iter().next().unwrap().clone()
}
