use bron_kerbosch;
use bron_kerbosch::graph::{Adjacencies, NewableUndirectedGraph, Vertex, VertexSetLike};

use rand::seq::SliceRandom;
use rand::Rng;

pub enum Order {
    Of(u32),
}
pub enum Size {
    Of(u32),
}

fn new_adjacencies<VertexSet>(order: u32) -> Adjacencies<VertexSet>
where
    VertexSet: VertexSetLike + Clone,
{
    std::vec::from_elem(VertexSet::new(), order as usize)
}

pub fn new_undirected<VertexSet, G>(rng: &mut impl Rng, order: Order, size: Size) -> G
where
    VertexSet: VertexSetLike + Clone,
    G: NewableUndirectedGraph<VertexSet>,
{
    let Order::Of(order) = order;
    let Size::Of(size) = size;
    let fully_meshed_size = order * (order - 1) / 2;
    assert!(order > 0);
    assert!(
        size <= fully_meshed_size,
        "{} nodes accommodate at most {} edges",
        order,
        fully_meshed_size
    );
    let mut unsaturated_vertices: Vec<Vertex> = (0..order as Vertex).collect();
    let mut adjacency_sets: Vec<VertexSet> = new_adjacencies(order);
    let mut adjacency_complements: Vec<VertexSet> = new_adjacencies(order);
    for _ in 0..size {
        debug_assert!(unsaturated_vertices
            .iter()
            .all(|&v| adjacency_sets[v as usize].len() < (order - 1) as usize));
        let v = *unsaturated_vertices.choose(rng).unwrap();
        let w = if !adjacency_complements[v as usize].is_empty() {
            *adjacency_complements[v as usize].choose(rng).unwrap()
        } else {
            loop {
                let w = *unsaturated_vertices.choose(rng).unwrap();
                if w != v && !adjacency_sets[v as usize].contains(w) {
                    break w;
                }
            }
        };
        debug_assert_ne!(v, w);
        debug_assert!(!adjacency_sets[v as usize].contains(w));
        debug_assert!(!adjacency_sets[w as usize].contains(v));
        for &(x, y) in &[(v, w), (w, v)] {
            adjacency_sets[x as usize].insert(y);
            let neighbours = adjacency_sets[x as usize].len() as u32;
            if neighbours == order - 1 {
                remove_from(&mut unsaturated_vertices, x);
                adjacency_complements[x as usize].clear();
            } else if neighbours == order / 2 {
                // start using adjacency complement
                debug_assert!(adjacency_complements[x as usize].is_empty());
                let mut s: VertexSet = unsaturated_vertices.iter().cloned().collect();
                s.remove(x);
                adjacency_complements[x as usize] =
                    s.difference_collect(&adjacency_sets[x as usize]);
            } else if neighbours > order / 2 {
                adjacency_complements[x as usize].remove(y);
            }
        }
    }
    let g = G::new(adjacency_sets);
    assert_eq!(g.order(), order);
    assert_eq!(g.size(), size);
    g
}

fn remove_from(vseq: &mut Vec<Vertex>, v: Vertex) {
    let index = vseq.iter().position(|&x| x == v).unwrap();
    vseq.swap_remove(index);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bron_kerbosch::slimgraph::SlimUndirectedGraph;

    use self::rand_chacha::ChaChaRng;
    use rand::SeedableRng;
    use rand_chacha;
    use std::collections::BTreeSet;

    #[test]
    fn random_graph() {
        type G = SlimUndirectedGraph<BTreeSet<Vertex>>;
        let mut rng = ChaChaRng::from_seed([68u8; 32]);
        let _: G = new_undirected(&mut rng, Order::Of(2), Size::Of(0));
        let _: G = new_undirected(&mut rng, Order::Of(3), Size::Of(0));
        let _: G = new_undirected(&mut rng, Order::Of(3), Size::Of(1));
        let _: G = new_undirected(&mut rng, Order::Of(3), Size::Of(2));
        let _: G = new_undirected(&mut rng, Order::Of(4), Size::Of(0));
        let _: G = new_undirected(&mut rng, Order::Of(4), Size::Of(1));
        let _: G = new_undirected(&mut rng, Order::Of(4), Size::Of(2));
        let _: G = new_undirected(&mut rng, Order::Of(4), Size::Of(3));
        let _: G = new_undirected(&mut rng, Order::Of(4), Size::Of(4));
        let _: G = new_undirected(&mut rng, Order::Of(4), Size::Of(5));
    }
}
