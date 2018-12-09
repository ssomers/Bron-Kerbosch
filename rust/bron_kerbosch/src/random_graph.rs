extern crate rand;
use self::rand::seq::{IteratorRandom, SliceRandom};
use self::rand::Rng;
use graph::{new_adjacencies, UndirectedGraph, Vertex};
use std::collections::HashSet;

pub enum Order {
    Of(u32),
}
pub enum Size {
    Of(u32),
}

pub fn new_undirected(rng: &mut impl Rng, order: Order, size: Size) -> UndirectedGraph {
    let Order::Of(order) = order;
    let Size::Of(size) = size;
    let fully_meshed_size = order * (order - 1) / 2;
    if size > fully_meshed_size {
        panic!(format!(
            "{} nodes accommodate at most {} edges",
            order, fully_meshed_size
        ));
    }
    let mut unsaturated_vertices: Vec<Vertex> = (0..order as Vertex).into_iter().collect();
    let mut adjacency_sets = new_adjacencies(order);
    let mut adjacency_complements = new_adjacencies(order);
    for _ in 0..size {
        let mut v: Vertex;
        let mut w: Vertex;
        v = *unsaturated_vertices.choose(rng).unwrap();
        assert!(adjacency_sets[v as usize].len() < (order - 1) as usize);
        if !adjacency_complements[v as usize].is_empty() {
            w = *adjacency_complements[v as usize]
                .iter()
                .choose(rng)
                .unwrap();
        } else {
            w = v;
            while w == v || adjacency_sets[v as usize].contains(&w) {
                w = *unsaturated_vertices.choose(rng).unwrap();
            }
        }
        assert_ne!(v, w);
        assert!(!adjacency_sets[v as usize].contains(&w));
        assert!(!adjacency_sets[w as usize].contains(&v));
        for (x, y) in vec![(v, w), (w, v)] {
            adjacency_sets[x as usize].insert(y);
            let neighbours = adjacency_sets[x as usize].len() as u32;
            if neighbours == order - 1 {
                let index = unsaturated_vertices.iter().position(|&v| v == x).unwrap();
                unsaturated_vertices.remove(index);
            } else if neighbours == order / 2 {
                // start using adjacency complement
                assert!(adjacency_complements[x as usize].is_empty());
                let mut s: HashSet<Vertex> = unsaturated_vertices.iter().cloned().collect();
                s.remove(&x);
                adjacency_complements[x as usize] =
                    s.difference(&adjacency_sets[x as usize]).cloned().collect();
            } else if neighbours > order / 2 {
                adjacency_complements[x as usize].remove(&y);
            }
        }
    }
    let g = UndirectedGraph::new(adjacency_sets);
    assert_eq!(g.order(), order);
    assert_eq!(g.size(), size);
    g
}
