use crate::{Vertex, VertexMap, VertexSetLike};
use rand::{Rng, prelude::IndexedRandom};
use std::ops::Not;

fn ordered<'a>(a: &'a Vec<Vertex>, b: &'a Vec<Vertex>) -> (&'a Vec<Vertex>, &'a Vec<Vertex>) {
    if a.len() < b.len() { (a, b) } else { (b, a) }
}

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

    fn difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Vertex> + 'a {
        debug_assert!(self.is_sorted());
        self.iter().filter(|&&v| other.contains(v).not())
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        debug_assert!(self.is_sorted());
        debug_assert!(other.is_sorted());
        let (small, big) = ordered(self, other);
        small.iter().any(|&v| big.contains(v)).not()
    }

    fn intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Vertex> + 'a {
        debug_assert!(self.is_sorted());
        debug_assert!(other.is_sorted());
        let (small, big) = ordered(self, other);
        small.iter().filter(|&&v| big.contains(v))
    }

    fn filter_map<'a>(&'a self, map: &'a VertexMap<bool>) -> impl Iterator<Item = &'a Vertex> + 'a {
        debug_assert!(self.is_sorted());
        self.iter().filter(|&v| map[*v])
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
