extern crate rand;
use std::collections::HashSet;

pub type Vertex = u32;
pub type Adjacencies = Vec<HashSet<Vertex>>;

pub fn new_adjacencies(order: u32) -> Adjacencies {
    std::vec::from_elem(HashSet::new(), order as usize)
}

pub struct UndirectedGraph {
    adjacencies: Adjacencies,
}

impl UndirectedGraph {
    pub fn assert_adjacencies(adjacencies: &Adjacencies) -> bool {
        for (i, adjacent_to_v) in adjacencies.iter().enumerate() {
            let v = i as Vertex;
            for &w in adjacent_to_v {
                debug_assert_ne!(v, w);
                debug_assert!(
                    adjacencies[w as usize].contains(&v),
                    format!("{} is adjacent to {} but not vice versa", w, v)
                );
            }
        }
        true
    }
    pub fn new(adjacencies: Adjacencies) -> Self {
        debug_assert!(UndirectedGraph::assert_adjacencies(&adjacencies));
        UndirectedGraph { adjacencies }
    }

    pub fn order(&self) -> u32 {
        self.adjacencies.len() as u32
    }

    pub fn size(&self) -> u32 {
        let total: u32 = self.adjacencies.iter().map(|a| a.len() as u32).sum();
        assert!(total % 2 == 0);
        total / 2
    }

    pub fn adjacencies(&self, node: Vertex) -> &HashSet<Vertex> {
        &self.adjacencies[node as usize]
    }
    pub fn degree(&self, node: Vertex) -> u32 {
        self.adjacencies(node).len() as u32
    }

    pub fn connected_nodes(&self) -> HashSet<Vertex> {
        (0..self.order() as Vertex)
            .filter(|&v| self.degree(v) > 0)
            .collect()
    }
}
