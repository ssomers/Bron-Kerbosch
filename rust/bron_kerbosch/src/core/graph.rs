pub use super::vertex::{Vertex, VertexMap};
pub use super::vertexsetlike::VertexSetLike;

pub trait UndirectedGraph: Sync {
    type VertexSet: VertexSetLike;

    fn order(&self) -> usize;
    fn size(&self) -> usize;
    fn degree(&self, node: Vertex) -> usize;
    fn neighbours(&self, node: Vertex) -> &Self::VertexSet;
}

pub fn connected_vertices<Graph>(g: &Graph) -> Graph::VertexSet
where
    Graph: UndirectedGraph,
{
    (0..g.order())
        .map(Vertex::new)
        .filter(|&v| g.degree(v) > 0)
        .collect()
}

pub type Adjacencies<VertexSet> = VertexMap<VertexSet>;

pub fn are_valid_adjacencies<VertexSet>(adjacencies: &Adjacencies<VertexSet>) -> bool
where
    VertexSet: VertexSetLike,
{
    let order = adjacencies.len();
    adjacencies.iter().all(|(v, adjacent_to_v)| {
        adjacent_to_v.all(|&w| w != v && w < Vertex::new(order) && adjacencies[w].contains(v))
    })
}

pub trait NewableUndirectedGraph<VertexSet>: UndirectedGraph<VertexSet = VertexSet> {
    fn new(adjacencies: Adjacencies<VertexSet>) -> Self;
}
