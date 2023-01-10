mod core;
#[cfg(test)]
mod degeneracy_tests;
mod vertexsetlikes;

pub use crate::core::graph::{Adjacencies, NewableUndirectedGraph, Vertex, VertexSetLike};
pub use crate::core::reporters::{CollectingReporter, CountingReporter};
pub use crate::core::slimgraph::SlimUndirectedGraph;
pub use crate::core::{explore, order_cliques, OrderedClique, OrderedCliques, FUNC_NAMES};

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
