use crate::SlimUndirectedGraph;
use crate::core::graph::{UndirectedGraph, Vertex, VertexSetLike, connected_vertices};
use crate::core::graph_degeneracy::degeneracy_iter;
use crate::core::slimgraph::Adjacencies;
use proptest::strategy::Strategy;
use std::collections::BTreeSet;

pub fn any_adjacencies<VertexSet: VertexSetLike + Clone>()
-> impl Strategy<Value = Adjacencies<VertexSet>> {
    (2..99usize).prop_flat_map(adjacency_input_of_order::<VertexSet>)
}

pub fn adjacency_input_of_order<VertexSet: VertexSetLike + Clone>(
    order: usize,
) -> impl Strategy<Value = Adjacencies<VertexSet>> {
    proptest::collection::vec(
        // The vector is indexed by a source vertex and lists a set of destination vertices.
        // We'll then reflect each vertex pair to make the adjacencies symmetric.
        proptest::collection::btree_set(0..order, ..order),
        // Symmetry implies there's no point in listing the last vertex already.
        order - 1,
    )
    .prop_map(move |adjac: Vec<BTreeSet<usize>>| {
        assert_eq!(order, adjac.len() + 1);
        let mut adjacencies = Adjacencies::new(VertexSet::new(), order);
        for (v, adjacent_to_v) in adjac.iter().enumerate() {
            for &w in adjacent_to_v.iter().filter(|&&w| w != v) {
                let w = Vertex::new(w);
                let v = Vertex::new(v);
                adjacencies[v].insert(w);
                adjacencies[w].insert(v);
            }
        }
        adjacencies
    })
}

pub fn test_degeneracy<VertexSet: VertexSetLike + Clone>(adjacencies: Adjacencies<VertexSet>) {
    let g = SlimUndirectedGraph::new(adjacencies);
    let connected: VertexSet = connected_vertices(&g).collect();

    let ordering = Vec::from_iter(degeneracy_iter(&g).map(|(v, _)| v));
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
    let mut ordering_and_neighbours = VertexSet::new();
    for v in ordering {
        ordering_and_neighbours.insert(v);
        g.neighbours(v)
            .for_each(|n| ordering_and_neighbours.insert(n))
    }
    assert_eq!(ordering_and_neighbours, connected);
}
