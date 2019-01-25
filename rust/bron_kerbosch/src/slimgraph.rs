use graph::{assert_adjacencies, Adjacencies, NewableUndirectedGraph, UndirectedGraph, Vertex};
use util::intersect;

use std::collections::HashSet;

#[derive(Debug)]
pub struct SlimUndirectedGraph {
    adjacencies: Adjacencies,
}

impl UndirectedGraph for SlimUndirectedGraph {
    fn order(&self) -> u32 {
        self.adjacencies.len() as u32
    }

    fn size(&self) -> u32 {
        let total: u32 = self.adjacencies.iter().map(|a| a.len() as u32).sum();
        assert!(total % 2 == 0);
        total / 2
    }

    fn degree(&self, node: Vertex) -> u32 {
        self.adjacencies[node as usize].len() as u32
    }

    fn neighbour_difference(&self, candidates: &HashSet<Vertex>, node: Vertex) -> Vec<Vertex> {
        candidates
            .difference(&self.adjacencies[node as usize])
            .cloned()
            .collect()
    }

    fn neighbour_intersection(&self, set: &HashSet<Vertex>, node: Vertex) -> HashSet<Vertex> {
        intersect(set, &self.adjacencies[node as usize])
            .cloned()
            .collect()
    }

    fn neighbour_intersection_count(&self, set: &HashSet<Vertex>, node: Vertex) -> usize {
        intersect(set, &self.adjacencies[node as usize]).count()
    }

    fn visit_neighbours<F>(&self, node: Vertex, mut f: F)
    where
        F: FnMut(Vertex),
    {
        for &v in self.adjacencies[node as usize].iter() {
            f(v);
        }
    }
}

impl NewableUndirectedGraph for SlimUndirectedGraph {
    fn new(adjacencies: Adjacencies) -> Self {
        debug_assert!(assert_adjacencies(&adjacencies));
        SlimUndirectedGraph { adjacencies }
    }
}
