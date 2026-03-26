use super::*;
use crate::core::vertexset_testing::graph_degeneracy_testing::{any_adjacencies, test_degeneracy};

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
    fn on_btree(adjacencies in any_adjacencies()) {
        test_degeneracy::<BTreeSet<Vertex>>(adjacencies);
    }

    #[test]
    fn on_hashset(adjacencies in any_adjacencies()) {
        test_degeneracy::<HashSet<Vertex>>(adjacencies);
    }

    #[test]
    fn on_fnv(adjacencies in any_adjacencies()) {
        test_degeneracy::<FnvHashSet<Vertex>>(adjacencies);
    }

    #[test]
    fn on_hashbrown(adjacencies in any_adjacencies()) {
        test_degeneracy::<hashbrown::HashSet<Vertex>>(adjacencies);
    }

    #[test]
    fn on_ordvec(adjacencies in any_adjacencies()) {
        test_degeneracy::<Vec<Vertex>>(adjacencies);
    }
}
