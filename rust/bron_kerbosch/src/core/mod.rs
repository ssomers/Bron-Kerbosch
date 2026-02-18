pub mod base;
mod bron_kerbosch1a;
mod bron_kerbosch1b;
mod bron_kerbosch2a_gp;
mod bron_kerbosch2b;
mod bron_kerbosch2b_gp;
mod bron_kerbosch2b_gpx;
mod bron_kerbosch2b_rp;
mod bron_kerbosch3_gp;
mod bron_kerbosch3_gpx;
mod bron_kerbosch3_mt;
mod bron_kerbosch_degen;
mod bron_kerbosch_degen_mt;
mod bron_kerbosch_pivot;
pub mod cliqueconsumers;
pub mod graph;
pub(super) mod graph_degeneracy;
mod pile;
pub mod slimgraph;
#[cfg(test)]
pub mod tests;
pub mod vertex;
pub mod vertexsetlike;

use base::{Clique, CliqueConsumer};
use graph::{UndirectedGraph, Vertex};
use std::collections::BTreeSet;

#[cfg(not(miri))]
const NUM_FUNCS: usize = 10;
#[cfg(miri)]
const NUM_FUNCS: usize = 9;
pub const FUNC_NAMES: &[&str; NUM_FUNCS] = &[
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

pub fn explore<Graph, Consumer>(func_index: usize, graph: &Graph, consumer: &mut Consumer)
where
    Graph: UndirectedGraph,
    Consumer: CliqueConsumer,
{
    match func_index {
        0 => bron_kerbosch1a::explore(graph, consumer),
        1 => bron_kerbosch1b::explore(graph, consumer),
        2 => bron_kerbosch2a_gp::explore(graph, consumer),
        3 => bron_kerbosch2b::explore(graph, consumer),
        4 => bron_kerbosch2b_gp::explore(graph, consumer),
        5 => bron_kerbosch2b_gpx::explore(graph, consumer),
        6 => bron_kerbosch2b_rp::explore(graph, consumer),
        7 => bron_kerbosch3_gp::explore(graph, consumer),
        8 => bron_kerbosch3_gpx::explore(graph, consumer),
        9 => bron_kerbosch3_mt::explore(graph, consumer),
        _ => panic!(),
    }
}

pub type OrderedClique = BTreeSet<Vertex>;
pub type OrderedCliques = BTreeSet<OrderedClique>;
pub fn order_cliques<I: Iterator<Item = Clique>>(cliques: I) -> OrderedCliques {
    BTreeSet::from_iter(cliques.map(BTreeSet::from_iter))
}
