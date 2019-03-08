use util::btree_intersect;

extern crate rand;
use self::rand::prelude::IteratorRandom;
use self::rand::Rng;
use std::collections::{BTreeSet, HashSet};
use std::iter::FromIterator;

pub type Vertex = u32;

pub trait VertexSetLike<VertexSet>: FromIterator<Vertex> {
    fn new() -> VertexSet;
    fn with_capacity(capacity: usize) -> VertexSet;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn contains(&self, v: &Vertex) -> bool;
    fn difference(&self, other: &VertexSet) -> Vec<Vertex>;
    fn has_same_elements(&self, vec: &Vec<Vertex>) -> bool;
    fn is_disjoint(&self, other: &VertexSet) -> bool;
    fn intersection_count(&self, other: &VertexSet) -> usize;
    fn intersection(&self, other: &VertexSet) -> VertexSet;
    fn intervection(&self, other: &VertexSet) -> Vec<Vertex>;
    fn reserve(&mut self, additional: usize);
    fn insert(&mut self, v: Vertex);
    fn remove(&mut self, v: &Vertex);
    fn pop_arbitrary(&mut self) -> Option<Vertex>;
    fn choose_arbitrary(&self) -> Option<&Vertex>;
    fn choose(&self, rng: &mut impl Rng) -> Option<&Vertex>;
    fn clear(&mut self);

    fn all<F>(&self, f: F) -> bool
    where
        F: Fn(&Vertex) -> bool;

    fn max_by_key_from_either<'a, F>(&'a self, excluded: &'a VertexSet, f: F) -> Option<&'a Vertex>
    where
        F: Fn(&&Vertex) -> usize;

    fn for_each<F>(&self, f: F)
    where
        F: FnMut(Vertex);
}

impl VertexSetLike<BTreeSet<Vertex>> for BTreeSet<Vertex> {
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
    fn difference(&self, other: &BTreeSet<Vertex>) -> Vec<Vertex> {
        self.difference(other).cloned().collect()
    }
    fn has_same_elements(&self, vec: &Vec<Vertex>) -> bool {
        let other: BTreeSet<Vertex> = vec.iter().cloned().collect();
        *self == other
    }
    fn is_disjoint(&self, other: &BTreeSet<Vertex>) -> bool {
        btree_intersect(self, other).next().is_none()
    }
    fn intersection_count(&self, other: &BTreeSet<Vertex>) -> usize {
        btree_intersect(self, other).count()
    }
    fn intersection(&self, other: &BTreeSet<Vertex>) -> BTreeSet<Vertex> {
        btree_intersect(self, other).cloned().collect()
    }
    fn intervection(&self, other: &BTreeSet<Vertex>) -> Vec<Vertex> {
        btree_intersect(self, other).cloned().collect()
    }
    fn reserve(&mut self, _additional: usize) {}
    fn insert(&mut self, v: Vertex) {
        self.insert(v);
    }
    fn remove(&mut self, v: &Vertex) {
        self.remove(v);
    }
    fn pop_arbitrary(&mut self) -> Option<Vertex> {
        match self.iter().next().cloned() {
            None => None,
            Some(elt) => self.take(&elt),
        }
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

    fn max_by_key_from_either<'a, F>(
        &'a self,
        excluded: &'a BTreeSet<Vertex>,
        f: F,
    ) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().chain(excluded).max_by_key(f)
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

impl VertexSetLike<HashSet<Vertex>> for HashSet<Vertex> {
    fn new() -> HashSet<Vertex> {
        HashSet::new()
    }
    fn with_capacity(capacity: usize) -> HashSet<Vertex> {
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
    fn difference(&self, other: &HashSet<Vertex>) -> Vec<Vertex> {
        self.difference(other).cloned().collect()
    }
    fn has_same_elements(&self, vec: &Vec<Vertex>) -> bool {
        let other: HashSet<Vertex> = vec.iter().cloned().collect();
        *self == other
    }
    fn is_disjoint(&self, other: &HashSet<Vertex>) -> bool {
        self.is_disjoint(other)
    }
    fn intersection_count(&self, other: &HashSet<Vertex>) -> usize {
        self.intersection(other).count()
    }
    fn intersection(&self, other: &HashSet<Vertex>) -> HashSet<Vertex> {
        self.intersection(other).cloned().collect()
    }
    fn intervection(&self, other: &HashSet<Vertex>) -> Vec<Vertex> {
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
        match self.iter().next().cloned() {
            None => None,
            Some(elt) => self.take(&elt),
        }
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

    fn max_by_key_from_either<'a, F>(
        &'a self,
        excluded: &'a HashSet<Vertex>,
        f: F,
    ) -> Option<&'a Vertex>
    where
        F: FnMut(&&Vertex) -> usize,
    {
        self.iter().chain(excluded).max_by_key(f)
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

pub trait UndirectedGraph<VertexSet>: Sync {
    fn order(&self) -> u32;
    fn size(&self) -> u32;
    fn degree(&self, node: Vertex) -> u32;
    fn neighbours(&self, node: Vertex) -> &VertexSet;
}

pub fn connected_nodes<VertexSet>(g: &UndirectedGraph<VertexSet>) -> VertexSet
where
    VertexSet: FromIterator<Vertex>,
{
    (0..g.order()).filter(|&v| g.degree(v) > 0).collect()
}

pub type Adjacencies<VertexSet> = Vec<VertexSet>;

pub fn new_adjacencies<VertexSet>(order: u32) -> Adjacencies<VertexSet>
where
    VertexSet: VertexSetLike<VertexSet> + Clone,
{
    std::vec::from_elem(VertexSet::with_capacity(0), order as usize)
}

pub fn assert_adjacencies<VertexSet>(adjacencies: &Adjacencies<VertexSet>) -> bool
where
    VertexSet: VertexSetLike<VertexSet>,
{
    for (i, adjacent_to_v) in adjacencies.iter().enumerate() {
        let v = i as Vertex;
        adjacent_to_v.all(|&w| w != v && adjacencies[w as usize].contains(&v));
    }
    true
}

pub trait NewableUndirectedGraph<VertexSet>: UndirectedGraph<VertexSet> {
    fn new(adjacencies: Adjacencies<VertexSet>) -> Self;
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
}
