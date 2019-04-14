extern crate fnv;
extern crate hashbrown;
extern crate rand;
use self::fnv::{FnvBuildHasher, FnvHashSet};
use self::rand::prelude::IteratorRandom;
use self::rand::Rng;
use graph::{Vertex, VertexSetLike};
use std::collections::{BTreeSet, HashSet};
use std::iter::FromIterator;

impl VertexSetLike for BTreeSet<Vertex> {
    fn new() -> BTreeSet<Vertex> {
        BTreeSet::new()
    }
    fn with_capacity(_capacity: usize) -> BTreeSet<Vertex> {
        BTreeSet::new()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn contains(&self, v: &Vertex) -> bool {
        self.contains(v)
    }
    fn difference<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.difference(other).cloned().collect()
    }
    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }
    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }
    fn intersection<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.intersection(other).cloned().collect()
    }
    fn reserve(&mut self, _additional: usize) {}
    fn insert(&mut self, v: Vertex) {
        self.insert(v);
    }
    fn remove(&mut self, v: &Vertex) {
        self.remove(v);
    }
    fn pop_arbitrary(&mut self) -> Option<Vertex> {
        let elt = self.iter().next().cloned()?;
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

    fn max_by_key<'a, F>(&'a self, f: F) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().max_by_key(f)
    }

    fn max_by_key_from_either<'a, F>(&'a self, other: &'a Self, f: F) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().chain(other).max_by_key(f)
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

impl VertexSetLike for HashSet<Vertex> {
    fn new() -> Self {
        HashSet::new()
    }
    fn with_capacity(capacity: usize) -> Self {
        HashSet::with_capacity(capacity)
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn contains(&self, v: &Vertex) -> bool {
        self.contains(v)
    }
    fn difference<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.difference(other).cloned().collect()
    }
    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }
    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }
    fn intersection<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.intersection(other).cloned().collect()
    }
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }
    fn insert(&mut self, v: Vertex) {
        self.insert(v);
    }
    fn remove(&mut self, v: &Vertex) {
        self.remove(v);
    }
    fn pop_arbitrary(&mut self) -> Option<Vertex> {
        let elt = self.iter().next().cloned()?;
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

    fn max_by_key<'a, F>(&'a self, f: F) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().max_by_key(f)
    }

    fn max_by_key_from_either<'a, F>(&'a self, other: &'a Self, f: F) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().chain(other).max_by_key(f)
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
        self.is_empty()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn contains(&self, v: &Vertex) -> bool {
        self.contains(v)
    }
    fn difference<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.difference(other).cloned().collect()
    }
    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }
    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }
    fn intersection<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.intersection(other).cloned().collect()
    }
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }
    fn insert(&mut self, v: Vertex) {
        self.insert(v);
    }
    fn remove(&mut self, v: &Vertex) {
        self.remove(v);
    }
    fn pop_arbitrary(&mut self) -> Option<Vertex> {
        let elt = self.iter().next().cloned()?;
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

    fn max_by_key<'a, F>(&'a self, f: F) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().max_by_key(f)
    }

    fn max_by_key_from_either<'a, F>(&'a self, other: &'a Self, f: F) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().chain(other).max_by_key(f)
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
    fn contains(&self, v: &Vertex) -> bool {
        self.contains(v)
    }
    fn difference<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.difference(other).cloned().collect()
    }
    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }
    fn intersection_size(&self, other: &Self) -> usize {
        self.intersection(other).count()
    }
    fn intersection<C>(&self, other: &Self) -> C
    where
        C: FromIterator<Vertex>,
    {
        self.intersection(other).cloned().collect()
    }
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }
    fn insert(&mut self, v: Vertex) {
        self.insert(v);
    }
    fn remove(&mut self, v: &Vertex) {
        self.remove(v);
    }
    fn pop_arbitrary(&mut self) -> Option<Vertex> {
        let elt = self.iter().next().cloned()?;
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

    fn max_by_key<'a, F>(&'a self, f: F) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().max_by_key(f)
    }

    fn max_by_key_from_either<'a, F>(&'a self, other: &'a Self, f: F) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().chain(other).max_by_key(f)
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

    #[test]
    fn test_btreeset_pop_arbitrary() {
        let mut s: BTreeSet<u32> = [4, 2].iter().cloned().collect();
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 1);
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 0);
        assert!(s.pop_arbitrary().is_none());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_hashset_pop_arbitrary() {
        let mut s: HashSet<u32> = [4, 2].iter().cloned().collect();
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 1);
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 0);
        assert!(s.pop_arbitrary().is_none());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_fnvhashset_pop_arbitrary() {
        let mut s: FnvHashSet<u32> = [4, 2].iter().cloned().collect();
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 1);
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 0);
        assert!(s.pop_arbitrary().is_none());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_hashbrownhashset_pop_arbitrary() {
        let mut s: hashbrown::HashSet<u32> = [4, 2].iter().cloned().collect();
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 1);
        assert!(s.pop_arbitrary().is_some());
        assert_eq!(s.len(), 0);
        assert!(s.pop_arbitrary().is_none());
        assert_eq!(s.len(), 0);
    }
}
