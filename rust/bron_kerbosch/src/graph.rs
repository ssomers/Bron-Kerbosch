extern crate rand;
use std::collections::HashSet;

pub type Vertex = u32;
pub type Adjacencies = Vec<HashSet<Vertex>>;

pub fn new_adjacencies(order: i32) -> Adjacencies {
    std::vec::from_elem(HashSet::new(), order as usize)
}

pub struct UndirectedGraph {
    adjacencies: Adjacencies,
}

impl UndirectedGraph {
    pub fn new(adjacencies: Adjacencies) -> Self {
        for (i, adjacent_to_v) in adjacencies.iter().enumerate() {
            let v = i as Vertex;
            for w in adjacent_to_v {
                assert_ne!(&v, w);
                assert!(
                    adjacencies[*w as usize].contains(&v),
                    format!("{} is adjacent to {} but not vice versa", w, v)
                );
            }
        }
        UndirectedGraph { adjacencies }
    }

    pub fn order(&self) -> i32 {
        self.adjacencies.len() as i32
    }

    pub fn size(&self) -> i32 {
        //total = sum(len(a) for a in self.adjacencies)
        let total: i32 = self.adjacencies.iter().map(|a| a.len() as i32).sum();
        assert!(total % 2 == 0);
        total / 2
    }

    pub fn adjacencies(&self, node: Vertex) -> &HashSet<Vertex> {
        &self.adjacencies[node as usize]
    }
    pub fn degree(&self, node: Vertex) -> i32 {
        self.adjacencies(node).len() as i32
    }

    pub fn connected_nodes(&self) -> HashSet<Vertex> {
        // {node for node in range(self.order) if self.adjacencies[node]}
        (0..self.order() as Vertex)
            .filter(|v| self.degree(*v) > 0)
            .collect()
    }
}
