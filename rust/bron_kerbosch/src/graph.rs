use std::collections::HashSet;

pub type Vertex = u32;
pub type VertexSet = HashSet<Vertex>;

pub trait UndirectedGraph: Sync {
    fn order(&self) -> u32;
    fn size(&self) -> u32;
    fn degree(&self, node: Vertex) -> u32;
    fn neighbours(&self, node: Vertex) -> &VertexSet;
}

pub fn connected_nodes(g: &UndirectedGraph) -> VertexSet {
    (0..g.order()).filter(|&v| g.degree(v) > 0).collect()
}

pub type Adjacencies = Vec<VertexSet>;

pub fn new_adjacencies(order: u32) -> Adjacencies {
    std::vec::from_elem(VertexSet::new(), order as usize)
}

pub fn assert_adjacencies(adjacencies: &Adjacencies) -> bool {
    for (i, adjacent_to_v) in adjacencies.iter().enumerate() {
        let v = i as Vertex;
        for &w in adjacent_to_v {
            assert_ne!(v, w);
            assert!(
                adjacencies[w as usize].contains(&v),
                "{} is adjacent to {} but not vice versa",
                w,
                v
            );
        }
    }
    true
}

pub trait NewableUndirectedGraph: UndirectedGraph {
    fn new(adjacencies: Adjacencies) -> Self;
}
