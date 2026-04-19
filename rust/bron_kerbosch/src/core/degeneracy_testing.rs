use super::degeneracy::Degeneracy;
use super::graph::Graph;
use super::vertexsetlike::VertexSetLike;

pub fn test_degeneracy<VertexSet: VertexSetLike>(g: Graph<VertexSet>) {
    let connected: VertexSet = g.connected_vertices().collect();

    let mut ordering = vec![];
    Degeneracy::on(&g).apply(|v, _| ordering.push(v));
    let ordering_set = VertexSet::from_iter(ordering.iter().copied());
    assert_eq!(ordering.len(), ordering_set.len(), "duplicates in ordering");
    if let Some(&first) = ordering.first() {
        assert!(g.degree(first) > 0);
        for &v in &ordering {
            assert!(g.degree(first) <= g.degree(v));
        }
        assert!(
            ordering.len() < connected.len(),
            "at least one vertex must remain with all its neighbours picked"
        );
    } else {
        assert!(connected.is_empty());
    }

    let mut ordering_and_neighbours = VertexSet::new();
    for v in ordering {
        ordering_and_neighbours.insert(v);
        g.neighbours(v)
            .for_each(|n| ordering_and_neighbours.insert(n))
    }
    assert_eq!(ordering_and_neighbours, connected);
}
