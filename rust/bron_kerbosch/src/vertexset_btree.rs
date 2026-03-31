use crate::{Vertex, VertexMap, VertexSetLike};

use rand::{Rng, seq::IteratorRandom};
use std::collections::BTreeSet;

impl VertexSetLike for BTreeSet<Vertex> {
    fn new() -> Self {
        BTreeSet::new()
    }

    fn with_capacity(_capacity: usize) -> Self {
        BTreeSet::new()
    }

    fn is_empty(&self) -> bool {
        BTreeSet::is_empty(self)
    }

    fn len(&self) -> usize {
        BTreeSet::len(self)
    }

    fn contains(&self, v: Vertex) -> bool {
        BTreeSet::contains(self, &v)
    }

    fn difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Vertex> + 'a {
        self.difference(other)
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).next().is_none()
    }

    fn intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Vertex> + 'a {
        self.intersection(other)
    }

    fn filter_map<'a>(&'a self, map: &'a VertexMap<bool>) -> impl Iterator<Item = &'a Vertex> + 'a {
        self.iter().filter(|&v| map[*v])
    }

    fn reserve(&mut self, _additional: usize) {}

    fn insert(&mut self, v: Vertex) {
        BTreeSet::insert(self, v);
    }

    fn remove(&mut self, v: Vertex) {
        BTreeSet::remove(self, &v);
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
        BTreeSet::clear(self)
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
