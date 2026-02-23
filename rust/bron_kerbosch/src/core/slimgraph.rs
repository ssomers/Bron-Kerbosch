use super::graph::{
    Adjacencies, NewableUndirectedGraph, UndirectedGraph, Vertex, VertexSetLike,
    are_valid_adjacencies,
};

#[derive(Debug)]
pub struct SlimUndirectedGraph<VertexSet: VertexSetLike> {
    my_adjacencies: Adjacencies<VertexSet>,
    my_size: usize,
    my_max_degree: usize,
}

impl<VertexSet: VertexSetLike> UndirectedGraph for SlimUndirectedGraph<VertexSet> {
    type VertexSet = VertexSet;

    fn order(&self) -> usize {
        self.my_adjacencies.len()
    }

    fn size(&self) -> usize {
        self.my_size
    }

    fn max_degree(&self) -> usize {
        self.my_max_degree
    }

    fn degree(&self, node: Vertex) -> usize {
        self.neighbours(node).len()
    }

    fn neighbours(&self, node: Vertex) -> &VertexSet {
        &self.my_adjacencies[node]
    }
}

impl<VertexSet> NewableUndirectedGraph<VertexSet> for SlimUndirectedGraph<VertexSet>
where
    VertexSet: VertexSetLike,
{
    fn new(adjacencies: Adjacencies<VertexSet>) -> Self {
        debug_assert!(are_valid_adjacencies(&adjacencies));
        let max_degree: usize = adjacencies.iter().map(|(_, a)| a.len()).max().unwrap_or(0);
        let sum_degree: usize = adjacencies.iter().map(|(_, a)| a.len()).sum();
        assert!(sum_degree.is_multiple_of(2));
        SlimUndirectedGraph {
            my_adjacencies: adjacencies,
            my_size: sum_degree / 2,
            my_max_degree: max_degree,
        }
    }
}
