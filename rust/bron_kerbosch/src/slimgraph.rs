use std::collections::HashSet;

use graph::{assert_adjacencies, Adjacencies, NewableUndirectedGraph, UndirectedGraph, Vertex};

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
        self.neighbours(node).len() as u32
    }

    fn neighbours(&self, node: Vertex) -> &HashSet<Vertex> {
        &self.adjacencies[node as usize]
    }
}

impl NewableUndirectedGraph for SlimUndirectedGraph {
    fn new(adjacencies: Adjacencies) -> Self {
        debug_assert!(assert_adjacencies(&adjacencies));
        SlimUndirectedGraph { adjacencies }
    }
}