pub use super::vertex::{Vertex, VertexMap};
pub use super::vertexsetlike::VertexSetLike;

pub trait UndirectedGraph: Sync {
    type VertexSet: VertexSetLike;

    fn order(&self) -> usize;
    fn size(&self) -> usize;
    fn max_degree(&self) -> usize;
    fn degree(&self, node: Vertex) -> usize;
    fn neighbours(&self, node: Vertex) -> &Self::VertexSet;
}

pub fn vertices<Graph>(g: &Graph) -> impl Iterator<Item = Vertex>
where
    Graph: UndirectedGraph,
{
    (0..g.order()).map(Vertex::new)
}

pub fn connected_vertices<Graph>(g: &Graph) -> impl Iterator<Item = Vertex>
where
    Graph: UndirectedGraph,
{
    vertices(g).filter(|&v| g.degree(v) > 0)
}

pub fn max_degree_vertices<Graph>(g: &Graph) -> impl Iterator<Item = Vertex>
where
    Graph: UndirectedGraph,
{
    vertices(g).filter(|&v| g.degree(v) == g.max_degree())
}
