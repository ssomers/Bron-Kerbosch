//! Naive Bron-Kerbosch algorithm
extern crate rand;

use graph::UndirectedGraph;
use graph::Vertex;
use reporter::Clique;
use reporter::Reporter;
use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates: HashSet<Vertex> = graph.connected_nodes();
    let excluded: HashSet<Vertex> = HashSet::new();
    let clique = vec![];
    visit(graph, reporter, candidates, excluded, clique);
}

fn visit(
    graph: &UndirectedGraph,
    reporter: &mut Reporter,
    mut candidates: HashSet<Vertex>,
    mut excluded: HashSet<Vertex>,
    clique: Clique,
) {
    reporter.inc_count();
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(&clique);
        return;
    }

    while !candidates.is_empty() {
        let v = candidates.iter().next().unwrap().clone();
        candidates.remove(&v);
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates: HashSet<Vertex> =
            neighbours.intersection(&candidates).cloned().collect();
        let neighbouring_excluded: HashSet<Vertex> =
            neighbours.intersection(&excluded).cloned().collect();
        visit(
            graph,
            reporter,
            neighbouring_candidates,
            neighbouring_excluded,
            [clique.as_slice(), &[v]].concat(),
        );
        excluded.insert(v);
    }
}
