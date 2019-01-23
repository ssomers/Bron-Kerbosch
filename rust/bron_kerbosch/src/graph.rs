use std::collections::HashSet;

pub type Vertex = u32;

pub trait UndirectedGraph: Sync {
    fn order(&self) -> u32;
    fn size(&self) -> u32;
    fn degree(&self, node: Vertex) -> u32;
    fn adjacencies(&self, node: Vertex) -> &HashSet<Vertex>;
    fn connected_nodes(&self) -> HashSet<Vertex>;
}

pub type Adjacencies = Vec<HashSet<Vertex>>;

pub fn new_adjacencies(order: u32) -> Adjacencies {
    std::vec::from_elem(HashSet::new(), order as usize)
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
