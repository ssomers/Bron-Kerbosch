use crate::core::vertexsetlike::{Vertex, VertexSetLike};

use fnv::{FnvBuildHasher, FnvHashSet};
use rand::{prelude::IndexedRandom, seq::IteratorRandom, Rng};
use std::collections::{BTreeSet, HashSet};

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

    fn difference_collect(&self, other: &Self) -> Self {
        self.difference(other).copied().collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        HashSet::is_disjoint(self, other)
    }

    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }

    fn intersection_collect(&self, other: &Self) -> Self {
        self.intersection(other).copied().collect()
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

impl VertexSetLike for hashbrown::HashSet<Vertex> {
    fn new() -> Self {
        hashbrown::HashSet::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        hashbrown::HashSet::with_capacity(capacity)
    }

    fn is_empty(&self) -> bool {
        hashbrown::HashSet::is_empty(self)
    }

    fn len(&self) -> usize {
        hashbrown::HashSet::len(self)
    }

    fn contains(&self, v: Vertex) -> bool {
        hashbrown::HashSet::contains(self, &v)
    }

    fn difference_collect(&self, other: &Self) -> Self {
        self.difference(other).copied().collect()
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        hashbrown::HashSet::is_disjoint(self, other)
    }

    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }

    fn intersection_collect(&self, other: &Self) -> Self {
        self.intersection(other).copied().collect()
    }

    fn reserve(&mut self, additional: usize) {
        hashbrown::HashSet::reserve(self, additional)
    }

    fn insert(&mut self, v: Vertex) {
        hashbrown::HashSet::insert(self, v);
    }

    fn remove(&mut self, v: Vertex) {
        hashbrown::HashSet::remove(self, &v);
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
        hashbrown::HashSet::clear(self)
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
        self.binary_search(&v).is_ok()
    }

    fn difference_collect(&self, other: &Self) -> Self {
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

    fn intersection_collect(&self, other: &Self) -> Self {
        if self.len() > other.len() {
            return other.intersection_collect(self);
        }
        self.iter()
            .filter(|&v| other.contains(*v))
            .copied()
            .collect()
    }

    fn reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional)
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
