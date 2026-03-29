use crate::VertexSetLike;
use crate::core::graph_degeneracy_testing::test_degeneracy;
use crate::core::lab_graphs::all_lab_graphs;

use std::collections::BTreeSet;
use std::collections::HashSet;

fn run_all<VertexSet: VertexSetLike>() {
    for td in all_lab_graphs::<VertexSet>() {
        test_degeneracy(td.graph);
    }
}

#[test]
fn on_btree() {
    run_all::<BTreeSet<_>>();
}

#[test]
fn on_hash() {
    run_all::<HashSet<_>>();
}

#[test]
fn on_fnv() {
    run_all::<fnv::FnvHashSet<_>>();
}

#[test]
fn on_hashbrown() {
    run_all::<hashbrown::HashSet<_>>();
}

#[test]
fn on_ordvec() {
    run_all::<Vec<_>>();
}
