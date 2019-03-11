//! Bron-Kerbosch algorithm with pivot and degeneracy ordering

use super::bron_kerbosch2;
use graph::{connected_nodes, UndirectedGraph, Vertex, VertexSetLike};
use reporter::Reporter;

use std::collections::HashMap;

pub fn explore<VertexSet>(graph: &UndirectedGraph<VertexSet>, reporter: &mut Reporter)
where
    VertexSet: VertexSetLike,
{
    let mut candidates = connected_nodes(graph);
    let mut excluded = VertexSet::with_capacity(candidates.len());
    let ordered = degeneracy_order(graph, &candidates);
    for v in ordered {
        let neighbours = graph.neighbours(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates = neighbours.intersection(&candidates);
        let neighbouring_excluded = neighbours.intersection(&excluded);
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

fn degeneracy_order<VertexSet>(graph: &UndirectedGraph<VertexSet>, nodes: &VertexSet) -> Vec<Vertex>
where
    VertexSet: VertexSetLike,
{
    // FIXME: can improve it to linear time
    let mut degrees: HashMap<Vertex, u32> = HashMap::new();
    nodes.for_each(|v| {
        let previous = degrees.insert(v, graph.degree(v));
        debug_assert!(previous.is_none());
    });
    let mut ordered: Vec<Vertex> = Vec::with_capacity(nodes.len());

    while !degrees.is_empty() {
        let i = *degrees.iter().min_by_key(|(&_v, &d)| d).unwrap().0;
        ordered.push(i);
        degrees.remove(&i);
        graph.neighbours(i).for_each(|v| {
            if let Some(d) = degrees.get_mut(&v) {
                *d -= 1;
            }
        });
    }
    debug_assert!(nodes.has_same_elements(&ordered));
    ordered
}
