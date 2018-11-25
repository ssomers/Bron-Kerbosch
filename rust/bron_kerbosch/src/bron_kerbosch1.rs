//! Naive Bron-Kerbosch algorithm
extern crate rand;

use graph::UndirectedGraph;
use graph::Vertex;
use reporter::Clique;
use reporter::Reporter;
use std::collections::HashSet;
use BronKerboschEngine;

pub struct ThisEngine {}

impl BronKerboschEngine for ThisEngine {
    fn explore(
        &self,
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
            assert!(!neighbours.is_empty());
            let mut extended_clique = clique.clone();
            extended_clique.push(v);
            let mut nearby_candidates: HashSet<Vertex> =
                candidates.intersection(&neighbours).cloned().collect();
            let mut nearby_excluded: HashSet<Vertex> =
                excluded.intersection(&neighbours).cloned().collect();
            self.explore(
                graph,
                extended_clique,
                &mut nearby_candidates,
                &mut nearby_excluded,
                reporter,
            );
            excluded.insert(v);
        }
    }
}
