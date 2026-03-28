pub use super::vertex::{Vertex, VertexMap};
pub use super::vertexsetlike::VertexSetLike;

pub trait GraphLike: Sync {
    type VertexSet: VertexSetLike;

    fn order(&self) -> usize;
    fn size(&self) -> usize;
    fn max_degree(&self) -> usize;
    fn degree(&self, v: Vertex) -> usize;
    fn neighbours(&self, v: Vertex) -> &Self::VertexSet;
}

pub fn vertices(g: &impl GraphLike) -> impl Iterator<Item = Vertex> {
    (0..g.order()).map(Vertex::new)
}

pub fn connected_vertices(g: &impl GraphLike) -> impl Iterator<Item = Vertex> {
    vertices(g).filter(|&v| g.degree(v) > 0)
}

pub fn max_degree_vertices(g: &impl GraphLike) -> impl Iterator<Item = Vertex> {
    vertices(g).filter(|&v| g.degree(v) == g.max_degree())
}
