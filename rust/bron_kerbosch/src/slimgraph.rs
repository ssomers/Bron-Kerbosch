use graph::{
    are_valid_adjacencies, Adjacencies, NewableUndirectedGraph, UndirectedGraph, Vertex,
    VertexSetLike,
};

#[derive(Debug)]
pub struct SlimUndirectedGraph<VertexSet>
where
    VertexSet: Sync,
{
    adjacencies: Adjacencies<VertexSet>,
}

impl<VertexSet> UndirectedGraph for SlimUndirectedGraph<VertexSet>
where
    VertexSet: VertexSetLike + Sync,
{
    type VertexSet = VertexSet;

    fn order(&self) -> usize {
        self.adjacencies.len()
    }

    fn size(&self) -> usize {
        let total: usize = self.adjacencies.iter().map(|(_, a)| a.len()).sum();
        assert!(total % 2 == 0);
        total / 2
    }

    fn degree(&self, node: Vertex) -> usize {
        self.neighbours(node).len()
    }

    fn neighbours(&self, node: Vertex) -> &VertexSet {
        &self.adjacencies[node]
    }
}

impl<VertexSet> NewableUndirectedGraph<VertexSet> for SlimUndirectedGraph<VertexSet>
where
    VertexSet: VertexSetLike + Sync,
{
    fn new(adjacencies: Adjacencies<VertexSet>) -> Self {
        debug_assert!(are_valid_adjacencies(&adjacencies));
        SlimUndirectedGraph { adjacencies }
    }
}
