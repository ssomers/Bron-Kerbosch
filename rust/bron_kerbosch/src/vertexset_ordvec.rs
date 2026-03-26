use crate::{Vertex, VertexSetLike};

use rand::{Rng, prelude::IndexedRandom};

impl VertexSetLike for Vec<Vertex> {
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn contains(&self, v: Vertex) -> bool {
        debug_assert!(self.is_sorted());
        self.binary_search(&v).is_ok()
    }

    fn difference_collect(&self, other: &Self) -> Self {
        debug_assert!(self.is_sorted());
        self.iter()
            .filter(|&v| !other.contains(*v))
            .copied()
            .collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        debug_assert!(self.is_sorted());
        if self.len() > other.len() {
            return other.is_disjoint(self);
        }
        !self.iter().any(|v| other.contains(*v))
    }

    fn intersection_size(&self, other: &Self) -> usize {
        debug_assert!(self.is_sorted());
        if self.len() > other.len() {
            return other.intersection_size(self);
        }
        self.iter().filter(|&v| other.contains(*v)).count()
    }

    fn intersection_collect(&self, other: &Self) -> Self {
        debug_assert!(self.is_sorted());
        if self.len() > other.len() {
            return other.intersection_collect(self);
        }
        self.iter()
            .filter(|&v| other.contains(*v))
            .copied()
            .collect()
    }

    fn intersection_with_fn_collect<F: Fn(Vertex) -> bool>(&self, other: F) -> Self {
        debug_assert!(self.is_sorted());
        self.iter().filter(|&v| other(*v)).copied().collect()
    }

    fn reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional)
    }

    fn insert(&mut self, v: Vertex) {
        debug_assert!(self.is_sorted());
        if let Err(idx) = self.binary_search(&v) {
            self.insert(idx, v);
            debug_assert!(self.is_sorted());
        }
    }

    fn remove(&mut self, v: Vertex) {
        debug_assert!(self.is_sorted());
        if let Ok(idx) = self.binary_search(&v) {
            self.remove(idx);
        }
    }

    fn pop_arbitrary(&mut self) -> Option<Vertex> {
        self.pop()
    }

    fn choose_arbitrary(&self) -> Option<&Vertex> {
        self.last()
    }

    fn choose(&self, rng: &mut impl Rng) -> Option<&Vertex> {
        self.as_slice().choose(rng)
    }

    fn clear(&mut self) {
        Vec::clear(self)
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
