use crate::{Vertex, VertexSetLike};

use fnv::FnvHashSet;
use std::collections::{BTreeSet, HashSet};

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
fn pop_arbitrary_btree() {
    test_pop_arbitrary::<BTreeSet<Vertex>>()
}

#[test]
fn pop_arbitrary_hashset() {
    test_pop_arbitrary::<HashSet<Vertex>>()
}

#[test]
fn pop_arbitrary_fnv() {
    test_pop_arbitrary::<FnvHashSet<Vertex>>()
}

#[test]
fn pop_arbitrary_hashbrown() {
    test_pop_arbitrary::<HashSet<Vertex>>()
}

#[test]
fn pop_arbitrary_vec() {
    test_pop_arbitrary::<Vec<Vertex>>()
}
