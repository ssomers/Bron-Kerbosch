//! Bron-Kerbosch algorithm with pivot and degeneracy ordering

use super::bron_kerbosch2;
use graph::{connected_nodes, UndirectedGraph, Vertex};
use reporter::Reporter;

use std::collections::{HashMap, HashSet};

pub fn explore(graph: &impl UndirectedGraph, reporter: &mut Reporter) {
    let mut candidates = connected_nodes(graph);
    let mut excluded = HashSet::with_capacity(candidates.len());
    let ordered = degeneracy_order(graph, &candidates);
    for v in ordered {
        let neighbouring_candidates = graph.neighbour_intersection(&candidates, v);
        let neighbouring_excluded = graph.neighbour_intersection(&excluded, v);
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

fn degeneracy_order(graph: &impl UndirectedGraph, nodes: &HashSet<Vertex>) -> Vec<Vertex> {
    // FIXME: can improve it to linear time
    let mut degrees: HashMap<Vertex, u32> = nodes.iter().map(|&v| (v, graph.degree(v))).collect();
    let mut ordered: Vec<Vertex> = Vec::with_capacity(nodes.len());

    while !degrees.is_empty() {
        let i = *degrees.iter().min_by_key(|(&_v, &d)| d).unwrap().0;
        ordered.push(i);
        degrees.remove(&i);
        graph.visit_neighbours(i, |v| {
            if let Some(d) = degrees.get_mut(&v) {
                *d -= 1;
            }
        });
    }
    debug_assert_eq!(ordered.iter().cloned().collect::<HashSet<Vertex>>(), *nodes);
    ordered
}
