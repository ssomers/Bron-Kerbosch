pub use super::graph::UndirectedGraph;
pub use super::vertex::VertexMap;
pub use super::vertexsetlike::VertexSetLike;

pub trait UndirectedGraphFactory<VertexSet>
where
    VertexSet: VertexSetLike,
{
    fn new(adjacencies: Adjacencies<VertexSet>) -> impl UndirectedGraph<VertexSet = VertexSet>;

    fn are_valid_adjacencies(adjacencies: &Adjacencies<VertexSet>) -> bool {
        adjacencies
            .iter()
            .all(|(v, neighbours)| neighbours.all(|&w| w != v && adjacencies[w].contains(v)))
        // adjacencies[w] confirms w is a valid index
    }
}

pub type Adjacencies<VertexSet> = VertexMap<VertexSet>;
