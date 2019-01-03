//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use bron_kerbosch_pivot::{visit, PivotChoice};
use graph::{UndirectedGraph, Vertex};
use pile::Pile;
use reporter::Reporter;
use util::intersect;

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    reporter.inc_count();
    let mut candidates = graph.connected_nodes();
    let mut excluded = HashSet::new();
    debug_assert_eq!(
        degeneracy_order_smart(graph, &candidates).collect::<HashSet<Vertex>>(),
        candidates
    );
    for v in degeneracy_order_smart(graph, &candidates) {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        candidates.remove(&v);
        let neighbouring_candidates = intersect(&neighbours, &candidates).cloned().collect();
        let neighbouring_excluded = intersect(neighbours, &excluded).cloned().collect();
        excluded.insert(v);
        visit(
            graph,
            reporter,
            PivotChoice::MaxDegree,
            PivotChoice::MaxDegree,
            neighbouring_candidates,
            neighbouring_excluded,
            Pile::Cons(&Pile::Empty, v),
        );
    }
}

#[derive(Debug)]
struct DegeneracyOrderIter<'a> {
    graph: &'a UndirectedGraph,
    infinite: u32,
    degree_per_node: Vec<u32>,
    nodes_per_degree: Vec<Vec<u32>>,
    num_left: usize,
}
impl<'a> DegeneracyOrderIter<'a> {
    fn pick_with_lowest_degree(&mut self) -> Vertex {
        debug_assert!(self
            .degree_per_node
            .iter()
            .enumerate()
            .all(|(v, &d)| d == self.infinite
                || self.nodes_per_degree[d as usize].contains(&(v as u32))));
        for d in 0..self.nodes_per_degree.len() {
            while let Some(v) = self.nodes_per_degree[d].pop() {
                if self.degree_per_node[v as usize] != self.infinite {
                    self.degree_per_node[v as usize] = self.infinite;
                    return v;
                }
                // else was moved to lower degree
            }
        }
        panic!("Should have returned");
    }
}

impl<'a> Iterator for DegeneracyOrderIter<'a> {
    type Item = Vertex;
    fn next(&mut self) -> Option<Vertex> {
        if self.num_left == 0 {
            None
        } else {
            self.num_left -= 1;
            let i = self.pick_with_lowest_degree();
            for &v in self.graph.adjacencies(i) {
                let d = self.degree_per_node[v as usize];
                if d != self.infinite {
                    self.degree_per_node[v as usize] = d - 1;
                    // move to lower degree, but no need to remove the original one
                    self.nodes_per_degree[(d - 1) as usize].push(v)
                }
            }
            Some(i)
        }
    }
}

fn degeneracy_order_smart<'a>(
    graph: &'a UndirectedGraph,
    candidates: &HashSet<Vertex>,
) -> DegeneracyOrderIter<'a> {
    let order = graph.order();
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
    DegeneracyOrderIter {
        graph,
        infinite,
        degree_per_node,
        nodes_per_degree,
        num_left: candidates.len(),
    }
}
