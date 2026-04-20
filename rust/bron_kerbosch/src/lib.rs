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

pub use core::clique::Clique;
pub use core::clique_consumer::CliqueConsumer;
pub use core::clique_ordering::{OrderedCliques, order_cliques};
pub use core::graph::{Adjacencies, Graph};
pub use core::vertex::{Vertex, VertexMap};
pub use core::vertexsetlike::VertexSetLike;

pub const FUNC_NAMES: &[&str] = &[
    "Ver1",
    "Ver1½",
    "Ver2-GP",
    "Ver2½",
    "Ver2½-GP",
    "Ver2½-GPX",
    "Ver2½-RP",
    "Ver3½-GP",
    "Ver3½-GPX",
    #[cfg(not(miri))]
    "Ver3½=GPc",
];

pub const FUNC_INDEX_MT: usize = 9;

pub fn explore<VertexSet, Consumer>(
    func_index: usize,
    graph: &Graph<VertexSet>,
    consumer: Consumer,
    num_visiting_threads: usize,
) -> Consumer::Harvest
where
    VertexSet: VertexSetLike + Sync,
    Consumer: CliqueConsumer + Clone + Send,
{
    assert!(num_visiting_threads >= 1);
    match func_index {
        0 => core::bron_kerbosch1a::explore(graph, consumer),
        1 => core::bron_kerbosch1b::explore(graph, consumer),
        2 => core::bron_kerbosch2a_gp::explore(graph, consumer),
        3 => core::bron_kerbosch2b::explore(graph, consumer),
        4 => core::bron_kerbosch2b_gp::explore(graph, consumer),
        5 => core::bron_kerbosch2b_gpx::explore(graph, consumer),
        6 => core::bron_kerbosch2b_rp::explore(graph, consumer),
        7 => core::bron_kerbosch3_gp::explore(graph, consumer),
        8 => core::bron_kerbosch3_gpx::explore(graph, consumer),
        FUNC_INDEX_MT => core::bron_kerbosch3_mt::explore(graph, consumer, num_visiting_threads),
        _ => panic!(),
    }
}
