mod core;
mod slimgraph;
mod vertexsetlikes;

#[cfg(test)]
mod graph_degeneracy_tests;
#[cfg(test)]
mod main_tests;
#[cfg(test)]
mod slimgraph_testing;

pub use core::clique::Clique;
pub use core::clique_harvester::{CliqueHarvester, new_clique_channel};
pub use core::clique_ordering::{OrderedCliques, order_cliques};
pub use core::graph::{UndirectedGraph, Vertex, VertexSetLike};
pub use core::{FUNC_NAMES, explore};
pub use slimgraph::{Adjacencies, SlimUndirectedGraph};
