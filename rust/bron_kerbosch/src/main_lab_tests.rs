use crate::clique_consumers::CliqueCollector;
use crate::core::lab_graphs::{LabGraph, all_lab_graphs};
use crate::{FUNC_INDEX_MT, FUNC_NAMES, VertexSetLike, explore, order_cliques};
use std::collections::BTreeSet;
use std::collections::HashSet;

fn run_serial<VertexSet: VertexSetLike + Sync>(td: &LabGraph<VertexSet>) {
    for (func_index, func_name) in FUNC_NAMES.iter().enumerate() {
        let cliques = explore(func_index, &td.graph, CliqueCollector::new(2), 1);
        let cliques = order_cliques(cliques.into_iter());
        assert_eq!(cliques, td.cliques, "for {} on {}", func_name, td.name);
    }
}

fn run_parallel<VertexSet: VertexSetLike + Sync>(td: &LabGraph<VertexSet>) {
    let cliques = explore(FUNC_INDEX_MT, &td.graph, CliqueCollector::new(2), 16);
    let cliques = order_cliques(cliques.into_iter());
    assert_eq!(
        cliques, td.cliques,
        "for {} on {}",
        FUNC_NAMES[FUNC_INDEX_MT], td.name
    );
}

fn run_all<VertexSet: VertexSetLike + Sync>() {
    for td in all_lab_graphs() {
        run_serial::<VertexSet>(&td);
        run_parallel::<VertexSet>(&td);
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
