//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{UndirectedGraph, Vertex};
use reporter::Reporter;
use vertex_stack::VertexStack;

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    reporter.inc_count();
    let mut candidates = graph.connected_nodes();
    let mut excluded = HashSet::new();
    let ordered = degeneracy_order_smart(graph, &candidates);
    for v in ordered {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        candidates.remove(&v);
        let neighbouring_candidates = neighbours.intersection(&candidates).cloned().collect();
        let neighbouring_excluded = neighbours.intersection(&excluded).cloned().collect();
        excluded.insert(v);
        visit(
            graph,
            reporter,
            PivotChoice::MaxDegree,
            PivotChoice::MaxDegree,
            neighbouring_candidates,
            neighbouring_excluded,
            VertexStack::Cons(&VertexStack::Empty, v),
        );
    }
}

fn pick_with_lowest_degree(
    degree_per_node: &Vec<u32>,
    nodes_per_degree: &mut Vec<Vec<u32>>,
    infinite: u32,
) -> Vertex {
    debug_assert!(degree_per_node
        .iter()
        .enumerate()
        .all(|(v, &d)| d == infinite || nodes_per_degree[d as usize].contains(&(v as u32))));
    for d in 0..nodes_per_degree.len() {
        while let Some(v) = nodes_per_degree[d].pop() {
            if degree_per_node[v as usize] != infinite {
                return v;
            }
            // else was moved to lower degree
        }
    }
    panic!("Should have returned");
}

fn degeneracy_order_smart(graph: &UndirectedGraph, candidates: &HashSet<Vertex>) -> Vec<Vertex> {
    let order: u32 = graph.order();
    let infinite: u32 = order * 2; // still >= order after decrementing in each iteration
    let mut degree_per_node: Vec<u32> = vec![infinite; order as usize];
    let mut max_degree: u32 = 0;
    for &node in candidates {
        let degree = graph.degree(node);
        assert!(degree > 0); // FYI, isolated nodes were excluded up front
        if max_degree < degree {
            max_degree = degree;
        }
        degree_per_node[node as usize] = degree;
    }
    let mut nodes_per_degree: Vec<Vec<u32>> = vec![vec![]; (max_degree + 1) as usize];
    for &node in candidates {
        let degree = graph.degree(node);
        nodes_per_degree[degree as usize].push(node);
    }

    let mut ordered: Vec<Vertex> = Vec::with_capacity(candidates.len());
    for _ in 0..candidates.len() {
        let i = pick_with_lowest_degree(&degree_per_node, &mut nodes_per_degree, infinite);
        degree_per_node[i as usize] = infinite;
        ordered.push(i);
        for &v in graph.adjacencies(i) {
            let d = degree_per_node[v as usize];
            if d != infinite {
                degree_per_node[v as usize] = d - 1;
                // move to lower degree, but no need to remove the original one
                nodes_per_degree[(d - 1) as usize].push(v)
            }
        }
    }
    debug_assert_eq!(
        ordered.iter().cloned().collect::<HashSet<Vertex>>(),
        *candidates
    );
    ordered
}
