use crate::{Vertex, VertexSetLike};

use fnv::{FnvBuildHasher, FnvHashSet};
use rand::{Rng, seq::IteratorRandom};

impl VertexSetLike for FnvHashSet<Vertex> {
    fn new() -> Self {
        FnvHashSet::default()
    }

    fn with_capacity(capacity: usize) -> Self {
        FnvHashSet::with_capacity_and_hasher(capacity, FnvBuildHasher::default())
    }

    fn is_empty(&self) -> bool {
        FnvHashSet::is_empty(self)
    }

    fn len(&self) -> usize {
        FnvHashSet::len(self)
    }

    fn contains(&self, v: Vertex) -> bool {
        FnvHashSet::contains(self, &v)
    }

    fn difference_collect(&self, other: &Self) -> Self {
        self.difference(other).copied().collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        FnvHashSet::is_disjoint(self, other)
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

    fn reserve(&mut self, additional: usize) {
        FnvHashSet::reserve(self, additional)
    }

    fn insert(&mut self, v: Vertex) {
        FnvHashSet::insert(self, v);
    }

    fn remove(&mut self, v: Vertex) {
        FnvHashSet::remove(self, &v);
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
        FnvHashSet::clear(self)
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
