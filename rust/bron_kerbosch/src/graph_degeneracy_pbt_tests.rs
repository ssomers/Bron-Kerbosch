use crate::Vertex;
use crate::core::degeneracy_testing::test_degeneracy;
use crate::graph_proptest_strategy::any_undirected_graph;

use proptest::prelude::*;
use std::collections::BTreeSet;
use std::collections::HashSet;

proptest! {
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
        test_degeneracy::<fnv::FnvHashSet<Vertex>>(g);
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
