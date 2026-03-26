use crate::core::graph::{UndirectedGraph, Vertex, VertexMap, VertexSetLike};

pub type Adjacencies<VertexSet> = VertexMap<VertexSet>;

#[derive(Debug)]
pub struct SlimUndirectedGraph<VertexSet: VertexSetLike> {
    adjacencies: Adjacencies<VertexSet>,
    size: usize,
    max_degree: usize,
}

impl<VertexSet: VertexSetLike> SlimUndirectedGraph<VertexSet> {
    pub fn are_valid_adjacencies(adjacencies: &Adjacencies<VertexSet>) -> bool {
        adjacencies
            .iter()
            .all(|(v, neighbours)| neighbours.all(|&w| w != v && adjacencies[w].contains(v)))
        // adjacencies[w] confirms that w is a valid index
    }

    pub fn new(adjacencies: Adjacencies<VertexSet>) -> Self {
        debug_assert!(Self::are_valid_adjacencies(&adjacencies));
        let max_degree: usize = adjacencies.iter().map(|(_, a)| a.len()).max().unwrap_or(0);
        let sum_degree: usize = adjacencies.iter().map(|(_, a)| a.len()).sum();
        assert!(sum_degree.is_multiple_of(2));
        SlimUndirectedGraph {
            adjacencies,
            size: sum_degree / 2,
            max_degree,
        }
    }
}

impl<VertexSet: VertexSetLike> UndirectedGraph for SlimUndirectedGraph<VertexSet> {
    type VertexSet = VertexSet;

    fn order(&self) -> usize {
        self.adjacencies.len()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn max_degree(&self) -> usize {
        self.max_degree
    }

    fn degree(&self, v: Vertex) -> usize {
        self.neighbours(v).len()
    }

    fn neighbours(&self, v: Vertex) -> &VertexSet {
        &self.adjacencies[v]
    }
}
