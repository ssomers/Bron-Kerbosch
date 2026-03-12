mod core;
#[cfg(test)]
mod graph_degeneracy_tests;
mod vertexsetlikes;

pub use core::clique::{Clique, CliqueConsumer};
pub use core::clique_collector::CliqueCollector;
pub use core::graph::{UndirectedGraph, Vertex, VertexSetLike};
pub use core::graphfactory::{Adjacencies, UndirectedGraphFactory};
pub use core::slimgraphfactory::SlimUndirectedGraphFactory;
pub use core::{FUNC_NAMES, OrderedClique, OrderedCliques, explore, order_cliques};

#[cfg(test)]
mod tests {
    use crate::core::tests::all_test_data;
    use fnv::FnvHashSet;
    use hashbrown;
    use std::collections::BTreeSet;
    use std::collections::HashSet;

    #[test]
    fn bk_btree() {
        for td in all_test_data() {
            td.run::<BTreeSet<_>>();
        }
    }

    #[test]
    fn bk_hash() {
        for td in all_test_data() {
            td.run::<HashSet<_>>();
        }
    }

    #[test]
    fn bk_fnv() {
        for td in all_test_data() {
            td.run::<FnvHashSet<_>>();
        }
    }

    #[test]
    fn bk_hashbrown() {
        for td in all_test_data() {
            td.run::<hashbrown::HashSet<_>>();
        }
    }
}
