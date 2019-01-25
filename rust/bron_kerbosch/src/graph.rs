use std::collections::HashSet;

pub type Vertex = u32;

pub trait UndirectedGraph: Sync {
    fn order(&self) -> u32;
    fn size(&self) -> u32;
    fn degree(&self, node: Vertex) -> u32;
    fn neighbour_difference(&self, candidates: &HashSet<Vertex>, node: Vertex) -> Vec<Vertex>;
    fn neighbour_intersection(&self, set: &HashSet<Vertex>, node: Vertex) -> HashSet<Vertex>;
    fn neighbour_intersection_count(&self, set: &HashSet<Vertex>, node: Vertex) -> usize;
    fn visit_neighbours<F>(&self, node: Vertex, f: F)
    where
        F: FnMut(Vertex);
}

pub fn connected_nodes(g: &impl UndirectedGraph) -> HashSet<Vertex> {
    (0..g.order()).filter(|&v| g.degree(v) > 0).collect()
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
