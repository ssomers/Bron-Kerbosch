extern crate rand;
extern crate rand_chacha;

mod graph;
pub mod random_graph;
pub mod reporter;

use graph::UndirectedGraph;
use graph::Vertex;
use reporter::Clique;
use reporter::Reporter;
use std::collections::HashSet;

pub fn bron_kerbosch1(
    graph: &UndirectedGraph,
    clique: Clique,
    candidates: &mut HashSet<Vertex>,
    excluded: &mut HashSet<Vertex>,
    reporter: &mut Reporter,
) {
    reporter.inc_count();
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(&clique);
    }

    while !candidates.is_empty() {
        let pivot = candidates.iter().next().unwrap().clone();
        candidates.remove(&pivot);
        let neighbours = graph.adjacencies(pivot);
        assert!(!neighbours.is_empty());
        let mut extended_clique = clique.clone();
        extended_clique.push(pivot);
        let mut nearby_candidates: HashSet<Vertex> =
            candidates.intersection(&neighbours).cloned().collect();
        let mut nearby_excluded: HashSet<Vertex> =
            excluded.intersection(&neighbours).cloned().collect();
        bron_kerbosch1(
            graph,
            extended_clique,
            &mut nearby_candidates,
            &mut nearby_excluded,
            reporter,
        );
        excluded.insert(pivot);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use random_graph::*;
    use reporter::SimpleReporter;
    use std::collections::BTreeSet;
    use std::collections::HashSet;

    fn bk(adjacencies: Vec<Vec<Vertex>>, expected_cliques: Vec<Clique>) {
        let adjacencies = adjacencies
            .iter()
            .map(|neighbours| neighbours.into_iter().cloned().collect())
            .collect();
        let graph = UndirectedGraph::new(adjacencies);
        // for func in funcs:
        if true {
            let mut reporter = SimpleReporter::new();
            let mut candidates = graph.connected_nodes().clone();
            let mut excluded = HashSet::<Vertex>::new();
            bron_kerbosch1(
                &graph,
                vec![],
                &mut candidates,
                &mut excluded,
                &mut reporter,
            );
            let current: BTreeSet<BTreeSet<Vertex>> = reporter
                .cliques
                .into_iter()
                .map(|clique| clique.into_iter().collect())
                .collect();
            let expected: BTreeSet<BTreeSet<Vertex>> = expected_cliques
                .into_iter()
                .map(|clique| clique.into_iter().collect())
                .collect();
            assert_eq!(current, expected);
        }
    }

    #[test]
    fn bk_order_0() {
        bk(vec![], vec![]);
    }

    #[test]
    fn bk_order_1() {
        bk(vec![vec![]], vec![]);
    }

    #[test]
    fn bk_order_2_isolated() {
        bk(vec![vec![], vec![]], vec![]);
    }

    #[test]
    fn bk_order_2_connected() {
        bk(vec![vec![1], vec![0]], vec![vec![0, 1]]);
    }

    #[test]
    fn bk_order_3_size_1() {
        bk(vec![vec![1], vec![0], vec![]], vec![vec![0, 1]]);
        bk(vec![vec![], vec![2], vec![1]], vec![vec![1, 2]]);
    }

    #[test]
    fn bk_order_3_size_2() {
        bk(
            vec![vec![1], vec![0, 2], vec![1]],
            vec![vec![0, 1], vec![1, 2]],
        );
    }

    #[test]
    fn bk_order_3_size_3() {
        bk(
            vec![vec![1, 2], vec![0, 2], vec![0, 1]],
            vec![vec![0, 1, 2]],
        );
    }

    #[test]
    fn bk_order_4_size_2_isolated() {
        bk(
            vec![vec![1, 2], vec![0], vec![0], vec![]],
            vec![vec![0, 1], vec![0, 2]],
        );
    }

    #[test]
    fn bk_order_4_size_2_connected() {
        bk(
            vec![vec![1], vec![0], vec![3], vec![2]],
            vec![vec![0, 1], vec![2, 3]],
        );
    }

    #[test]
    fn bk_order_4_size_4_p() {
        bk(
            vec![vec![1], vec![0, 2, 3], vec![1, 3], vec![1, 2]],
            vec![vec![0, 1], vec![1, 2, 3]],
        );
    }

    #[test]
    fn bk_order_4_size_4_square() {
        bk(
            vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]],
            vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![2, 3]],
        );
    }

    #[test]
    fn bk_order_4_size_4_square_diagonal() {
        bk(
            vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]],
            vec![vec![0, 1, 2], vec![0, 2, 3]],
        );
    }

    #[test]
    fn bk_sample() {
        bk(
            vec![
                vec![],
                vec![2, 3, 4],
                vec![1, 3, 4, 5],
                vec![1, 2, 4, 5],
                vec![1, 2, 3],
                vec![2, 3, 6, 7],
                vec![5, 7],
                vec![5, 6],
            ],
            vec![vec![1, 2, 3, 4], vec![2, 3, 5], vec![5, 6, 7]],
        );
    }

    #[test]
    fn random_graph() {
        let mut rng = ChaChaRng::from_seed([68u8; 32]);
        new_undirected(&mut rng, Order::Of(2), Size::Of(0));
        new_undirected(&mut rng, Order::Of(3), Size::Of(0));
        new_undirected(&mut rng, Order::Of(3), Size::Of(1));
        new_undirected(&mut rng, Order::Of(3), Size::Of(2));
        new_undirected(&mut rng, Order::Of(4), Size::Of(0));
        new_undirected(&mut rng, Order::Of(4), Size::Of(1));
        new_undirected(&mut rng, Order::Of(4), Size::Of(2));
        new_undirected(&mut rng, Order::Of(4), Size::Of(3));
        new_undirected(&mut rng, Order::Of(4), Size::Of(4));
        new_undirected(&mut rng, Order::Of(4), Size::Of(5));
    }
}
