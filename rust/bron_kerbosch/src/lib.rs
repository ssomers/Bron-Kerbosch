pub mod clique_consumers;
mod core;
mod vertexset_btree;
mod vertexset_fnv;
mod vertexset_hashbrown;
mod vertexset_hashset;
mod vertexset_ordvec;

#[cfg(test)]
mod graph_degeneracy_lab_tests;
#[cfg(all(test, not(miri)))]
mod graph_degeneracy_pbt_tests;
#[cfg(test)]
mod graph_proptest_strategy;
#[cfg(test)]
mod main_lab_tests;
#[cfg(test)]
mod vertexset_tests;

pub use core::algorithm::BronKerboschAlgorithm;
pub use core::clique::Clique;
pub use core::clique_consumer::CliqueConsumer;
pub use core::clique_ordering::{OrderedCliques, order_cliques};
pub use core::graph::{Adjacencies, Graph};
pub use core::vertex::{Vertex, VertexMap};
pub use core::vertexsetlike::VertexSetLike;

pub const NUM_FUNCS: usize = 18;

macro_rules! algo_select {
    ($index: ident, $f: ident, $args: tt) => {
        match $index {
            0 => core::bron_kerbosch1a::Algo::$f$args,
            1 => core::bron_kerbosch1b::Algo::$f$args,
            2 => core::bron_kerbosch2a_gp::Algo::$f$args,
            3 => core::bron_kerbosch2b::Algo::$f$args,
            4 => core::bron_kerbosch2b_gp::Algo::$f$args,
            5 => core::bron_kerbosch2b_gpx::Algo::$f$args,
            6 => core::bron_kerbosch2b_rp::Algo::$f$args,
            7 => core::bron_kerbosch3_gp::Algo::$f$args,
            8 => core::bron_kerbosch3_gpx::Algo::$f$args,
            9 => core::bron_kerbosch3_mt::Algo::<1>::$f$args,
            10 => core::bron_kerbosch3_mt::Algo::<2>::$f$args,
            11 => core::bron_kerbosch3_mt::Algo::<3>::$f$args,
            12 => core::bron_kerbosch3_mt::Algo::<4>::$f$args,
            13 => core::bron_kerbosch3_mt::Algo::<5>::$f$args,
            14 => core::bron_kerbosch3_mt::Algo::<6>::$f$args,
            15 => core::bron_kerbosch3_mt::Algo::<8>::$f$args,
            16 => core::bron_kerbosch3_mt::Algo::<24>::$f$args,
            17 => core::bron_kerbosch3_mt::Algo::<72>::$f$args,
            _ => panic!(),
        }
    };
}

pub fn algo_name(func_index: usize) -> String {
    algo_select!(func_index, name, ())
}

pub fn algo_deterministic(func_index: usize) -> bool {
    algo_select!(func_index, deterministic, ())
}

pub fn algo_explore<VertexSet, Consumer>(
    func_index: usize,
    graph: &Graph<VertexSet>,
    consumer: Consumer,
) -> Consumer::Harvest
where
    VertexSet: VertexSetLike + Sync,
    Consumer: CliqueConsumer + Clone + Send,
{
    algo_select!(func_index, explore, (graph, consumer))
}

#[test]
fn algo_names_are_unique() {
    use std::collections::BTreeSet;
    assert_eq!(
        (0..NUM_FUNCS).map(algo_name).collect::<BTreeSet<_>>().len(),
        NUM_FUNCS,
        "algorithm names are not unique"
    );
}
