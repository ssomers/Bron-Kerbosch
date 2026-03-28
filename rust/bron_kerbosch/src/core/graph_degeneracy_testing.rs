use super::graph::Graph;
use super::graph_degeneracy::degeneracy_iter;
use super::vertexsetlike::VertexSetLike;

pub fn test_degeneracy<VertexSet: VertexSetLike + Clone>(g: Graph<VertexSet>) {
    let connected: VertexSet = g.connected_vertices().collect();

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
