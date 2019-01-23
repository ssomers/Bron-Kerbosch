extern crate bron_kerbosch;
use bron_kerbosch::graph::{new_adjacencies, NewableUndirectedGraph, Vertex};

extern crate rand;
use self::rand::seq::{IteratorRandom, SliceRandom};
use self::rand::Rng;
use std::collections::HashSet;

pub enum Order {
    Of(u32),
}
pub enum Size {
    Of(u32),
}

pub fn new_undirected<G: NewableUndirectedGraph>(
    rng: &mut impl Rng,
    order: Order,
    size: Size,
) -> G {
    let Order::Of(order) = order;
    let Size::Of(size) = size;
    let fully_meshed_size = order * (order - 1) / 2;
    if size > fully_meshed_size {
        panic!(
            "{} nodes accommodate at most {} edges",
            order, fully_meshed_size
        );
    }
    let mut unsaturated_vertices: Vec<Vertex> = (0..order as Vertex).into_iter().collect();
    let mut adjacency_sets = new_adjacencies(order);
    let mut adjacency_complements = new_adjacencies(order);
    for _ in 0..size {
        let mut v: Vertex;
        let mut w: Vertex;
        v = *unsaturated_vertices.choose(rng).unwrap();
        debug_assert!(adjacency_sets[v as usize].len() < (order - 1) as usize);
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
        debug_assert_ne!(v, w);
        debug_assert!(!adjacency_sets[v as usize].contains(&w));
        debug_assert!(!adjacency_sets[w as usize].contains(&v));
        for (x, y) in vec![(v, w), (w, v)] {
            adjacency_sets[x as usize].insert(y);
            let neighbours = adjacency_sets[x as usize].len() as u32;
            if neighbours == order - 1 {
                let index = unsaturated_vertices.iter().position(|&v| v == x).unwrap();
                unsaturated_vertices.remove(index);
            } else if neighbours == order / 2 {
                // start using adjacency complement
                debug_assert!(adjacency_complements[x as usize].is_empty());
                let mut s: HashSet<Vertex> = unsaturated_vertices.iter().cloned().collect();
                s.remove(&x);
                adjacency_complements[x as usize] =
                    s.difference(&adjacency_sets[x as usize]).cloned().collect();
            } else if neighbours > order / 2 {
                adjacency_complements[x as usize].remove(&y);
            }
        }
    }
    let g = G::new(adjacency_sets);
    assert_eq!(g.order(), order);
    assert_eq!(g.size(), size);
    g
}

#[cfg(test)]
mod tests {
    use super::*;
    use bron_kerbosch::slimgraph::SlimUndirectedGraph;

    extern crate rand_chacha;
    use self::rand_chacha::ChaChaRng;
    use rand::SeedableRng;

    #[test]
    fn random_graph() {
        let mut rng = ChaChaRng::from_seed([68u8; 32]);
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(2), Size::Of(0));
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(3), Size::Of(0));
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(3), Size::Of(1));
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(3), Size::Of(2));
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(4), Size::Of(0));
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(4), Size::Of(1));
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(4), Size::Of(2));
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(4), Size::Of(3));
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(4), Size::Of(4));
        let _: SlimUndirectedGraph = new_undirected(&mut rng, Order::Of(4), Size::Of(5));
    }
}
