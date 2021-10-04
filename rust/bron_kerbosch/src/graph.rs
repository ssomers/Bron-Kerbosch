pub use crate::vertex::{Vertex, VertexMap};

use rand::Rng;
use std::fmt::Debug;
use std::iter::FromIterator;

pub trait VertexSetLike: Eq + Debug + FromIterator<Vertex> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn contains(&self, v: Vertex) -> bool;
    fn difference_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>;
    fn is_disjoint(&self, other: &Self) -> bool;
    fn intersection_size(&self, other: &Self) -> usize;
    fn intersection_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>;
    fn reserve(&mut self, additional: usize);
    fn insert(&mut self, v: Vertex);
    fn remove(&mut self, v: Vertex);
    fn pop_arbitrary(&mut self) -> Option<Vertex>;
    fn choose_arbitrary(&self) -> Option<&Vertex>;
    fn choose(&self, rng: &mut impl Rng) -> Option<&Vertex>;
    fn clear(&mut self);

    fn all<F>(&self, f: F) -> bool
    where
        F: Fn(&Vertex) -> bool;

    fn for_each<F>(&self, f: F)
    where
        F: FnMut(Vertex);
}

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

pub trait NewableUndirectedGraph<VertexSet>: UndirectedGraph {
    fn new(adjacencies: Adjacencies<VertexSet>) -> Self;
}
