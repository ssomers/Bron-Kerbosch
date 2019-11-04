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

    fn neighbours(&self, node: Vertex) -> &VertexSet {
        &self.adjacencies[node as usize]
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
