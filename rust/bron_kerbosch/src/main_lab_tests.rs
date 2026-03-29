use crate::core::lab_graphs::{LabGraph, all_lab_graphs};
use crate::{FUNC_NAMES, VertexSetLike, explore, new_clique_channel, order_cliques};
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::thread;

fn run<VertexSet: VertexSetLike>(td: LabGraph<VertexSet>) {
    for (func_index, func_name) in FUNC_NAMES.iter().enumerate() {
        let (consumer, collector) = new_clique_channel(0, 2);
        let cliques = thread::scope(|s| {
            s.spawn(|| explore(func_index, &td.graph, consumer));
            collector.collect_cliques()
        });
        let cliques = order_cliques(cliques.into_iter());
        assert_eq!(cliques, td.cliques, "for {} on {}", func_name, td.name);
    }
}

fn run_all<VertexSet: VertexSetLike>() {
    for td in all_lab_graphs() {
        run::<VertexSet>(td);
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
