use super::vertex::{Vertex, VertexMap};
use super::vertexsetlike::VertexSetLike;
use std::ops::Not;

pub type Adjacencies<VertexSet> = VertexMap<VertexSet>;

#[derive(Debug)]
pub struct Graph<VertexSet: VertexSetLike> {
    adjacencies: Adjacencies<VertexSet>,
    size: usize,
    max_degree: usize,
}

impl<VertexSet: VertexSetLike> Graph<VertexSet> {
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
        Graph {
            adjacencies,
            size: sum_degree / 2,
            max_degree,
        }
    }
}

impl<VertexSet: VertexSetLike> Graph<VertexSet> {
    pub fn order(&self) -> usize {
        self.adjacencies.len()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn max_degree(&self) -> usize {
        self.max_degree
    }

    pub fn is_connected(&self, v: Vertex) -> bool {
        self.neighbours(v).is_empty().not()
    }

    pub fn degree(&self, v: Vertex) -> usize {
        self.neighbours(v).len()
    }

    pub fn neighbours(&self, v: Vertex) -> &VertexSet {
        &self.adjacencies[v]
    }

    pub fn vertices(&self) -> impl Iterator<Item = Vertex> {
        (0..self.order()).map(Vertex::new)
    }

    pub fn connected_vertices(&self) -> impl Iterator<Item = Vertex> {
        self.vertices().filter(|&v| self.is_connected(v))
    }

    pub fn max_degree_vertices(&self) -> impl Iterator<Item = Vertex> {
        let max = self.max_degree();
        self.vertices().filter(move |&v| self.degree(v) == max)
    }
}
