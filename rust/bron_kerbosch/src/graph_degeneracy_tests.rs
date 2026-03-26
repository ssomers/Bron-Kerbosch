use super::*;
use crate::core::vertexset_testing::graph_degeneracy_testing::test_degeneracy;
use crate::slimgraph_testing::any_undirected_graph;

use fnv::FnvHashSet;
use hashbrown;
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;
use std::collections::BTreeSet;
use std::collections::HashSet;

#[cfg(not(miri))]
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1968,
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        .. ProptestConfig::default()
    })]

    #[test]
    fn on_btree(g in any_undirected_graph()) {
        test_degeneracy::<BTreeSet<Vertex>>(g);
    }

    #[test]
    fn on_hashset(g in any_undirected_graph()) {
        test_degeneracy::<HashSet<Vertex>>(g);
    }

    #[test]
    fn on_fnv(g in any_undirected_graph()) {
        test_degeneracy::<FnvHashSet<Vertex>>(g);
    }

    #[test]
    fn on_hashbrown(g in any_undirected_graph()) {
        test_degeneracy::<hashbrown::HashSet<Vertex>>(g);
    }

    #[test]
    fn on_ordvec(g in any_undirected_graph()) {
        test_degeneracy::<Vec<Vertex>>(g);
    }
}
