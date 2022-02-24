pub use super::vertex::Vertex;

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
