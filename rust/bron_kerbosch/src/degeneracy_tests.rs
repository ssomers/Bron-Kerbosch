use super::*;
use crate::core::graph::{UndirectedGraph, connected_vertices};
use crate::core::graph_degeneracy::degeneracy_iter;
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};
use std::iter::once;

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
            let connected: VertexSet = connected_vertices(&g).collect();

            let ordering = Vec::from_iter(degeneracy_iter(&g));
            let ordering_set = VertexSet::from_iter(ordering.iter().copied());
            assert_eq!(ordering.len(), ordering_set.len(), "duplicates in ordering");
            if let Some(&first) = ordering.first() {
                for &v in &ordering {
                    assert!(g.degree(first) <= g.degree(v));
                }
            }

            assert!(
                ordering.len() < connected.len().max(1),
                "at least one vertex must remain with all its neighbours seen"
            );
            let ordering_and_neighbours: VertexSet = ordering
                .iter()
                .flat_map(|v| once(v).chain(g.neighbours(*v)))
                .copied()
                .collect();
            assert_eq!(ordering_and_neighbours, connected);

            Ok(())
        },
    )
    .unwrap();
}
