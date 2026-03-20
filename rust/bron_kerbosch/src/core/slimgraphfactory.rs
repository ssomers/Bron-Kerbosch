use super::graph::{UndirectedGraph, Vertex, VertexSetLike};
use super::graphfactory::{Adjacencies, UndirectedGraphFactory};

pub struct SlimUndirectedGraphFactory();

impl<VertexSet> UndirectedGraphFactory<VertexSet> for SlimUndirectedGraphFactory
where
    VertexSet: VertexSetLike,
{
    fn new(adjacencies: Adjacencies<VertexSet>) -> impl UndirectedGraph<VertexSet = VertexSet> {
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

#[derive(Debug)]
struct SlimUndirectedGraph<VertexSet: VertexSetLike> {
    adjacencies: Adjacencies<VertexSet>,
    size: usize,
    max_degree: usize,
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
