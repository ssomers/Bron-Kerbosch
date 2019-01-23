mod bron_kerbosch1;
mod bron_kerbosch1o;
mod bron_kerbosch2;
mod bron_kerbosch2_gp;
mod bron_kerbosch2_gpx;
mod bron_kerbosch2_rp;
mod bron_kerbosch2o;
mod bron_kerbosch3;
mod bron_kerbosch3o;
mod bron_kerbosch3om;
mod bron_kerbosch_degeneracy;
mod bron_kerbosch_pivot;
pub mod graph;
mod pile;
pub mod reporter;
pub mod util;

use graph::UndirectedGraph;
use graph::Vertex;
use reporter::Clique;
use reporter::{Reporter, SimpleReporter};
use std::collections::BTreeSet;

pub const NUM_FUNCS: usize = 10;
pub static FUNC_NAMES: &'static [&str; NUM_FUNCS] = &[
    "Ver1", "Ver1+", "Ver2", "Ver2+", "Ver2_RP", "Ver2_GP", "Ver2_GPX", "Ver3", "Ver3+", "Ver3+MT",
];
pub static FUNCS: &'static [fn(graph: &UndirectedGraph, reporter: &mut Reporter); NUM_FUNCS] = &[
    bron_kerbosch1::explore,
    bron_kerbosch1o::explore,
    bron_kerbosch2::explore,
    bron_kerbosch2o::explore,
    bron_kerbosch2_rp::explore,
    bron_kerbosch2_gp::explore,
    bron_kerbosch2_gpx::explore,
    bron_kerbosch3::explore,
    bron_kerbosch3o::explore,
    bron_kerbosch3om::explore,
];

pub type OrderedClique = BTreeSet<Vertex>;
pub type OrderedCliques = BTreeSet<OrderedClique>;
pub fn order_cliques(cliques: Vec<Clique>) -> OrderedCliques {
    cliques
        .into_iter()
        .map(|clique| clique.into_iter().collect())
        .collect()
}

pub fn bron_kerbosch(graph: &UndirectedGraph) -> OrderedCliques {
    let mut first: Option<OrderedCliques> = None;
    for func in FUNCS {
        let mut reporter = SimpleReporter::new();
        func(&graph, &mut reporter);
        let current = order_cliques(reporter.cliques);
        if first.is_none() {
            first = Some(current);
        } else {
            assert_eq!(current, *first.as_ref().unwrap());
        }
    }
    first.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use reporter::Clique;

    fn bk(adjacencies: Vec<Vec<Vertex>>, expected_cliques: Vec<Clique>) {
        let adjacencies = adjacencies
            .iter()
            .map(|neighbours| neighbours.into_iter().cloned().collect())
            .collect();
        let graph = UndirectedGraph::new(adjacencies);
        let current = bron_kerbosch(&graph);
        assert_eq!(current, order_cliques(expected_cliques));
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
    fn bk_order_4_size_5() {
        bk(
            vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]],
            vec![vec![0, 1, 2], vec![0, 2, 3]],
        );
    }

    #[test]
    fn bk_order_4_size_6() {
        bk(
            vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]],
            vec![vec![0, 1, 2, 3]],
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
}
