use crate::{Adjacencies, Graph, Vertex, VertexSetLike};
use proptest::strategy::Strategy;
use std::collections::BTreeSet;

pub fn any_undirected_graph<VertexSet: VertexSetLike + Clone>()
-> impl Strategy<Value = Graph<VertexSet>> {
    (2..99usize).prop_flat_map(undirected_graph_of_order::<VertexSet>)
}

fn undirected_graph_of_order<VertexSet: VertexSetLike + Clone>(
    order: usize,
) -> impl Strategy<Value = Graph<VertexSet>> {
    proptest::collection::vec(
        // The vector is indexed by a source vertex and lists a set of destination vertices,
        // where a destination vertex ≥ the source vertex is listed as one less.
        // We'll reflect each vertex pair to make the adjacencies symmetric.
        proptest::collection::btree_set(0..(order - 1), ..order),
        // Symmetry implies there's no point in listing the last source vertex here.
        order - 1,
    )
    .prop_map(move |adjac: Vec<BTreeSet<usize>>| {
        assert_eq!(order, adjac.len() + 1);
        let mut adjacencies = Adjacencies::new(VertexSet::new(), order);
        for (v, adjacent_to_v) in adjac.iter().enumerate() {
            for &w in adjacent_to_v {
                let w = Vertex::new(if w < v { w } else { w + 1 });
                let v = Vertex::new(v);
                adjacencies[v].insert(w);
                adjacencies[w].insert(v);
            }
        }
        adjacencies
    })
    .prop_map(|adjacencies| Graph::new(adjacencies))
}
