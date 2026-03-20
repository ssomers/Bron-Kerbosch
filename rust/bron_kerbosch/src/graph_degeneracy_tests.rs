use super::*;
use crate::core::graph_degeneracy_testing::test_degeneracy_order;

use fnv::FnvHashSet;
use hashbrown;
use std::collections::BTreeSet;
use std::collections::HashSet;

#[cfg(not(miri))]
#[test]
pub fn test_degeneracy_order_btree() {
    test_degeneracy_order::<BTreeSet<Vertex>>();
}

#[cfg(not(miri))]
#[test]
pub fn test_degeneracy_order_hash() {
    test_degeneracy_order::<HashSet<Vertex>>();
}

#[cfg(not(miri))]
#[test]
pub fn test_degeneracy_order_fnv() {
    test_degeneracy_order::<FnvHashSet<Vertex>>();
}

#[cfg(not(miri))]
#[test]
pub fn test_degeneracy_order_hashbrown() {
    test_degeneracy_order::<hashbrown::HashSet<Vertex>>();
}

#[cfg(not(miri))]
#[test]
pub fn test_degeneracy_order_ordvec() {
    test_degeneracy_order::<Vec<Vertex>>();
}
