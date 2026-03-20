mod core;
mod main_tests;
mod vertexsetlikes;

pub use core::clique::Clique;
pub use core::clique_harvester::{CliqueHarvester, new_clique_channel};
pub use core::clique_ordering::{OrderedCliques, order_cliques};
pub use core::graph::{UndirectedGraph, Vertex, VertexSetLike};
pub use core::graphfactory::{Adjacencies, UndirectedGraphFactory};
pub use core::slimgraphfactory::SlimUndirectedGraphFactory;
pub use core::{FUNC_NAMES, explore};

#[cfg(test)]
mod graph_degeneracy_tests;
