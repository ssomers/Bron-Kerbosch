use crate::{Vertex, VertexSetLike};

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

    fn difference_collect(&self, other: &Self) -> Self {
        self.difference(other).copied().collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).next().is_none()
    }

    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }

    fn intersection_collect(&self, other: &Self) -> Self {
        self.intersection(other).copied().collect()
    }

    fn intersection_with_fn_collect<F: Fn(Vertex) -> bool>(&self, other: F) -> Self {
        self.iter().filter(|&v| other(*v)).copied().collect()
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
