use super::*;
use crate::core::graph::{
    connected_vertices, Adjacencies, NewableUndirectedGraph, UndirectedGraph, Vertex,
};
use crate::core::graph_degeneracy::degeneracy_ordering;
use proptest::prelude::*;
use proptest::test_runner::TestRunner;

type VertexSet = std::collections::BTreeSet<Vertex>;

#[cfg(not(miri))]
#[test]
pub fn test_degeneracy_order() {
    TestRunner::default()
        .run(
            &(2..99usize).prop_flat_map(|order| {
                proptest::collection::vec(
                    proptest::collection::btree_set(0..order - 1, ..order),
                    order,
                )
            }),
            |adjac| {
                let order = adjac.len();
                let adjacencies = Vec::from_iter((0..order).map(|_| VertexSet::new()));
                let mut adjacencies = Adjacencies::sneak_in(adjacencies);
                for (v, adjacent_to_v) in adjac.iter().enumerate() {
                    let v = Vertex::new(v);
                    for &w in adjacent_to_v {
                        let w = Vertex::new(w);
                        if w != v {
                            adjacencies[v].insert(w);
                            adjacencies[w].insert(v);
                        }
                    }
                }

                let g = SlimUndirectedGraph::new(adjacencies);
                let connected = connected_vertices(&g);

                let ordering = Vec::from_iter(degeneracy_ordering(&g, 0));
                let ordering_set = VertexSet::from_iter(ordering.iter().copied());
                assert_eq!(ordering.len(), ordering_set.len(), "duplicates in ordering");
                assert_eq!(ordering_set, connected);
                if let Some(&first) = ordering.first() {
                    for &v in &ordering {
                        assert!(g.degree(first) <= g.degree(v));
                    }
                }

                let orderin = Vec::from_iter(degeneracy_ordering(&g, -1));
                assert_eq!(orderin, ordering[..connected.len().saturating_sub(1)]);
                Ok(())
            },
        )
        .unwrap();
}
