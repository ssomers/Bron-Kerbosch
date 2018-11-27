//! Naive Bron-Kerbosch algorithm
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
    reporter.inc_count();
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(&clique);
    }

    while !candidates.is_empty() {
        let v = candidates.iter().next().unwrap().clone();
        candidates.remove(&v);
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
        excluded.insert(v);
    }
}
