use crate::core::main_testing::all_test_data;
use fnv::FnvHashSet;
use hashbrown;
use std::collections::BTreeSet;
use std::collections::HashSet;

#[test]
fn on_btree() {
    for td in all_test_data() {
        td.run::<BTreeSet<_>>();
    }
}

#[test]
fn on_hash() {
    for td in all_test_data() {
        td.run::<HashSet<_>>();
    }
}

#[test]
fn on_fnv() {
    for td in all_test_data() {
        td.run::<FnvHashSet<_>>();
    }
}

#[test]
fn on_hashbrown() {
    for td in all_test_data() {
        td.run::<hashbrown::HashSet<_>>();
    }
}

#[test]
fn on_ordvec() {
    for td in all_test_data() {
        td.run::<Vec<_>>();
    }
}
