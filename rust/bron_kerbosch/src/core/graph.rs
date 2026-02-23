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

pub trait NewableUndirectedGraph<VertexSet>: UndirectedGraph<VertexSet = VertexSet> {
    fn new(adjacencies: Adjacencies<VertexSet>) -> Self;
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

pub type Adjacencies<VertexSet> = VertexMap<VertexSet>;

pub fn are_valid_adjacencies<VertexSet>(adjacencies: &Adjacencies<VertexSet>) -> bool
where
    VertexSet: VertexSetLike,
{
    adjacencies
        .iter()
        .all(|(v, neighbours)| neighbours.all(|&w| w != v && adjacencies[w].contains(v)))
    // adjacencies[w] confirms w is a valid index
}
