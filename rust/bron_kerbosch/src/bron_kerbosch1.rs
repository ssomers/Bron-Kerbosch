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
    mut candidates: HashSet<Vertex>,
    mut excluded: HashSet<Vertex>,
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
        let neighbouring_candidates: HashSet<Vertex> =
            candidates.intersection(&neighbours).cloned().collect();
        let neighbouring_excluded: HashSet<Vertex> =
            excluded.intersection(&neighbours).cloned().collect();
        explore(
            graph,
            [clique.as_slice(), &[v]].concat(),
            neighbouring_candidates,
            neighbouring_excluded,
            reporter,
        );
        excluded.insert(v);
    }
}
