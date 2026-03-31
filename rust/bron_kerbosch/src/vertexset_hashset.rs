use crate::{Vertex, VertexMap, VertexSetLike};

use rand::{Rng, seq::IteratorRandom};
use std::collections::HashSet;

#[allow(clippy::implicit_hasher)]
impl VertexSetLike for HashSet<Vertex> {
    fn new() -> Self {
        HashSet::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        HashSet::with_capacity(capacity)
    }

    fn is_empty(&self) -> bool {
        HashSet::is_empty(self)
    }

    fn len(&self) -> usize {
        HashSet::len(self)
    }

    fn contains(&self, v: Vertex) -> bool {
        HashSet::contains(self, &v)
    }

    fn difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Vertex> + 'a {
        self.difference(other)
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        HashSet::is_disjoint(self, other)
    }

    fn intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Vertex> + 'a {
        self.intersection(other)
    }

    fn filter_map<'a>(&'a self, map: &'a VertexMap<bool>) -> impl Iterator<Item = &'a Vertex> + 'a {
        self.iter().filter(|&v| map[*v])
    }

    fn reserve(&mut self, additional: usize) {
        HashSet::reserve(self, additional)
    }

    fn insert(&mut self, v: Vertex) {
        HashSet::insert(self, v);
    }

    fn remove(&mut self, v: Vertex) {
        HashSet::remove(self, &v);
    }

    fn pop_arbitrary(&mut self) -> Option<Vertex> {
        let elt = self.iter().next().copied()?;
        self.take(&elt)
    }

    fn choose_arbitrary(&self) -> Option<&Vertex> {
        self.iter().next()
    }

    fn choose(&self, rng: &mut impl Rng) -> Option<&Vertex> {
        self.iter().choose(rng)
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn all<F>(&self, f: F) -> bool
    where
        F: FnMut(&Vertex) -> bool,
    {
        self.iter().all(f)
    }

    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Vertex),
    {
        for &v in self {
            f(v);
        }
    }
}
