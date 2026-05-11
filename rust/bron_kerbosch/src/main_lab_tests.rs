use crate::clique_consumers::CliqueCollector;
use crate::core::lab_graphs::all_lab_graphs;
use crate::{NUM_FUNCS, VertexSetLike, algo_deterministic, algo_explore, algo_name, order_cliques};
use std::collections::BTreeSet;
use std::collections::HashSet;

fn run_all<VertexSet: VertexSetLike + Sync>() {
    for func_index in 0..NUM_FUNCS {
        let func_name = algo_name(func_index);
        let tries = if algo_deterministic(func_index) {
            1
        } else {
            11 // we also have 9 versions, each differing only by thread count
        };
        for td in all_lab_graphs::<VertexSet>() {
            for _ in 0..tries {
                let cliques = algo_explore(func_index, &td.graph, CliqueCollector::new(2));
                let cliques = order_cliques(cliques.into_iter());
                assert_eq!(cliques, td.cliques, "for {} on {}", func_name, td.name);
            }
        }
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
