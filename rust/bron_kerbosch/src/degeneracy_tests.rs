use super::*;
use crate::core::graph::{connected_vertices, UndirectedGraph};
use crate::core::graph_degeneracy::degeneracy_ordering;
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};

type VertexSet = std::collections::BTreeSet<Vertex>;

#[cfg(not(miri))]
#[test]
pub fn test_degeneracy_order() {
    TestRunner::new(Config {
        cases: 1968,
        ..Config::default()
    })
    .run(
        &(2..99usize).prop_flat_map(|order| {
            proptest::collection::vec(
                // The vector is indexed by a source vertex and lists a set of destination vertices,
                // in which we'll skip any vertex numbers less than or equal to the source vertex.
                // We'll then reflect each vertex pair to make the adjacencies symmetric.
                proptest::collection::btree_set(0..order, ..order - 1),
                // No point in listing the last vertex, it cannot contribute more neighbours.
                order - 1,
            )
        }),
        |adjac| {
            let order = adjac.len() + 1;
            let mut adjacencies = Adjacencies::new(VertexSet::new(), order);
            for (v, adjacent_to_v) in adjac.iter().enumerate() {
                for &w in adjacent_to_v {
                    if w > v {
                        let w = Vertex::new(w);
                        let v = Vertex::new(v);
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
