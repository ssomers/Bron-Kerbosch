use crate::core::vertexsetlike::{Vertex, VertexSetLike};

use fnv::{FnvBuildHasher, FnvHashSet};
use rand::{seq::IteratorRandom, seq::SliceRandom, Rng};
use std::collections::{BTreeSet, HashSet};
use std::iter::FromIterator;

impl VertexSetLike for BTreeSet<Vertex> {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, v: Vertex) -> bool {
        self.contains(&v)
    }

    fn difference_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.difference(other).copied().collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).next().is_none()
    }

    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }

    fn intersection_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.intersection(other).copied().collect()
    }

    fn reserve(&mut self, _additional: usize) {}

    fn insert(&mut self, v: Vertex) {
        self.insert(v);
    }

    fn remove(&mut self, v: Vertex) {
        self.remove(&v);
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
        self.clear();
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

#[allow(clippy::implicit_hasher)]
impl VertexSetLike for HashSet<Vertex> {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, v: Vertex) -> bool {
        self.contains(&v)
    }

    fn difference_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.difference(other).copied().collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }

    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }

    fn intersection_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.intersection(other).copied().collect()
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn insert(&mut self, v: Vertex) {
        self.insert(v);
    }

    fn remove(&mut self, v: Vertex) {
        self.remove(&v);
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
        self.clear();
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

impl VertexSetLike for FnvHashSet<Vertex> {
    fn new() -> Self {
        Self::default()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FnvBuildHasher::default())
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, v: Vertex) -> bool {
        self.contains(&v)
    }

    fn difference_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.difference(other).copied().collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }

    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }

    fn intersection_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.intersection(other).copied().collect()
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn insert(&mut self, v: Vertex) {
        self.insert(v);
    }

    fn remove(&mut self, v: Vertex) {
        self.remove(&v);
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
        self.clear();
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

impl VertexSetLike for hashbrown::HashSet<Vertex> {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, v: Vertex) -> bool {
        self.contains(&v)
    }

    fn difference_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.difference(other).copied().collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }

    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }

    fn intersection_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.intersection(other).copied().collect()
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn insert(&mut self, v: Vertex) {
        self.insert(v);
    }

    fn remove(&mut self, v: Vertex) {
        self.remove(&v);
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
        self.clear();
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

impl VertexSetLike for Vec<Vertex> {
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, v: Vertex) -> bool {
        self.binary_search(&v).is_ok()
    }

    fn difference_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.iter()
            .filter(|&v| !other.contains(*v))
            .copied()
            .collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        if self.len() > other.len() {
            return other.is_disjoint(self);
        }
        !self.iter().any(|v| other.contains(*v))
    }

    fn intersection_size(&self, other: &Self) -> usize {
        if self.len() > other.len() {
            return other.intersection_size(self);
        }
        self.iter().filter(|&v| other.contains(*v)).count()
    }

    fn intersection_collect<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        if self.len() > other.len() {
            return other.intersection_collect(self);
        }
        self.iter()
            .filter(|&v| other.contains(*v))
            .copied()
            .collect()
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional)
    }

    fn insert(&mut self, v: Vertex) {
        if let Err(idx) = self.binary_search(&v) {
            self.insert(idx, v);
        }
    }

    fn remove(&mut self, v: Vertex) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pop_arbitrary<VertexSet: VertexSetLike>() {
        let mut s = VertexSet::new();
        s.insert(Vertex::new(4));
        s.insert(Vertex::new(2));
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 1);
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 0);
        assert!(s.pop_arbitrary().is_none());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_btreeset_pop_arbitrary() {
        test_pop_arbitrary::<BTreeSet<Vertex>>()
    }

    #[test]
    fn test_hashset_pop_arbitrary() {
        test_pop_arbitrary::<HashSet<Vertex>>()
    }

    #[test]
    fn test_fnvhashset_pop_arbitrary() {
        test_pop_arbitrary::<FnvHashSet<Vertex>>()
    }

    #[test]
    fn test_hashbrownhashset_pop_arbitrary() {
        test_pop_arbitrary::<HashSet<Vertex>>()
    }

    #[test]
    fn test_vector_pop_arbitrary() {
        test_pop_arbitrary::<Vec<Vertex>>()
    }
}
