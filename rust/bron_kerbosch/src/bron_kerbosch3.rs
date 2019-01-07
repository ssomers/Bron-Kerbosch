//! Bron-Kerbosch algorithm with pivot and degeneracy ordering

use super::bron_kerbosch2;
use graph::{UndirectedGraph, Vertex};
use reporter::Reporter;
use util::intersect;

use std::collections::{HashMap, HashSet};

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let mut candidates = graph.connected_nodes();
    let mut excluded = HashSet::with_capacity(candidates.len());
    let ordered = degeneracy_order(graph, &candidates);
    for v in ordered {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates = intersect(&neighbours, &candidates).cloned().collect();
        let neighbouring_excluded = intersect(&neighbours, &excluded).cloned().collect();
        bron_kerbosch2::visit(
            graph,
            reporter,
            neighbouring_candidates,
            neighbouring_excluded,
            vec![v],
        );
        candidates.remove(&v);
        excluded.insert(v);
    }
}

fn degeneracy_order(graph: &UndirectedGraph, nodes: &HashSet<Vertex>) -> Vec<Vertex> {
    // FIXME: can improve it to linear time
    let mut degrees: HashMap<Vertex, u32> = nodes.iter().map(|&v| (v, graph.degree(v))).collect();
    let mut ordered: Vec<Vertex> = Vec::with_capacity(nodes.len());

    while !degrees.is_empty() {
        let i = *degrees.iter().min_by_key(|(&_v, &d)| d).unwrap().0;
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
