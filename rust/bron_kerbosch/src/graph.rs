extern crate fnv;
extern crate hashbrown;
extern crate rand;
use self::rand::Rng;
use std::fmt::Debug;
use std::iter::FromIterator;

pub type Vertex = u32;

pub trait VertexSetLike: Eq + Debug + FromIterator<Vertex> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn contains(&self, v: &Vertex) -> bool;
    fn difference<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>;
    fn is_disjoint(&self, other: &Self) -> bool;
    fn intersection_size(&self, other: &Self) -> usize;
    fn intersection<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>;
    fn reserve(&mut self, additional: usize);
    fn insert(&mut self, v: Vertex);
    fn remove(&mut self, v: &Vertex);
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

pub trait UndirectedGraph<VertexSet>: Sync {
    fn order(&self) -> u32;
    fn size(&self) -> u32;
    fn degree(&self, node: Vertex) -> u32;
    fn neighbours(&self, node: Vertex) -> &VertexSet;
}

pub fn connected_vertices<VertexSet>(g: &UndirectedGraph<VertexSet>) -> VertexSet
where
    VertexSet: FromIterator<Vertex>,
{
    (0..g.order()).filter(|&v| g.degree(v) > 0).collect()
}

pub type Adjacencies<VertexSet> = Vec<VertexSet>;

pub fn new_adjacencies<VertexSet>(order: u32) -> Adjacencies<VertexSet>
where
    VertexSet: VertexSetLike + Clone,
{
    std::vec::from_elem(VertexSet::with_capacity(0), order as usize)
}

pub fn are_valid_adjacencies<VertexSet>(adjacencies: &Adjacencies<VertexSet>) -> bool
where
    VertexSet: VertexSetLike,
{
    let order = adjacencies.len() as u32;
    adjacencies
        .iter()
        .enumerate()
        .map(|(i, neighbours)| (i as Vertex, neighbours))
        .all(|(v, adjacent_to_v)| {
            adjacent_to_v.all(|&w| w != v && w < order && adjacencies[w as usize].contains(&v))
        })
}

pub trait NewableUndirectedGraph<VertexSet>: UndirectedGraph<VertexSet> {
    fn new(adjacencies: Adjacencies<VertexSet>) -> Self;
}
