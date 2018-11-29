//! Bron-Kerbosch algorithm with pivot and degeneracy ordering
extern crate rand;

use super::bron_kerbosch2;
use graph::UndirectedGraph;
use graph::Vertex;
use reporter::Clique;
use reporter::Reporter;
use std::collections::HashMap;
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
        assert!(clique.is_empty());
        return;
    }

    let ordered = degeneracy_order(graph, &candidates);
    for v in ordered {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates: HashSet<Vertex> =
            neighbours.intersection(&candidates).cloned().collect();
        let neighbouring_excluded: HashSet<Vertex> =
            neighbours.intersection(&excluded).cloned().collect();
        bron_kerbosch2::explore(
            graph,
            [clique.as_slice(), &[v]].concat(),
            neighbouring_candidates,
            neighbouring_excluded,
            reporter,
        );
        candidates.remove(&v);
        excluded.insert(v);
    }
}

fn degeneracy_order(graph: &UndirectedGraph, nodes: &HashSet<Vertex>) -> Vec<Vertex> {
    // FIXME: can improve it to linear time
    let mut degrees: HashMap<Vertex, u32> = nodes.iter().map(|v| (*v, graph.degree(*v))).collect();
    let mut ordered: Vec<Vertex> = Vec::with_capacity(nodes.len());

    while !degrees.is_empty() {
        let i = *degrees.iter().min_by_key(|(_v, d)| *d).unwrap().0;
        ordered.push(i);
        degrees.remove(&i);
        for v in graph.adjacencies(i) {
            if let Some(d) = degrees.get_mut(v) {
                *d -= 1;
            }
        }
    }
    debug_assert_eq!(ordered.iter().cloned().collect::<HashSet<Vertex>>(), *nodes);
    ordered
}
