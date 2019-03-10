// #![feature(test)]
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
pub mod slimgraph;
pub mod util;

use graph::{UndirectedGraph, Vertex, VertexSetLike};
use reporter::{Clique, Reporter, SimpleReporter};
use std::collections::BTreeSet;

pub const NUM_FUNCS: usize = 10;
pub static FUNC_NAMES: &'static [&str; NUM_FUNCS] = &[
    "Ver1", "Ver1+", "Ver2", "Ver2+", "Ver2_RP", "Ver2_GP", "Ver2_GPX", "Ver3", "Ver3+", "Ver3+MT",
];

pub fn explore<VertexSet>(
    func_index: usize,
    graph: &UndirectedGraph<VertexSet>,
    reporter: &mut Reporter,
) where
    VertexSet: VertexSetLike<VertexSet> + Send,
{
    match func_index {
        0 => bron_kerbosch1::explore(graph, reporter),
        1 => bron_kerbosch1o::explore(graph, reporter),
        2 => bron_kerbosch2::explore(graph, reporter),
        3 => bron_kerbosch2o::explore(graph, reporter),
        4 => bron_kerbosch2_rp::explore(graph, reporter),
        5 => bron_kerbosch2_gp::explore(graph, reporter),
        6 => bron_kerbosch2_gpx::explore(graph, reporter),
        7 => bron_kerbosch3::explore(graph, reporter),
        8 => bron_kerbosch3o::explore(graph, reporter),
        9 => bron_kerbosch3om::explore(graph, reporter),
        _ => panic!(),
    }
}

pub type OrderedClique = BTreeSet<Vertex>;
pub type OrderedCliques = BTreeSet<OrderedClique>;
pub fn order_cliques(cliques: Vec<Clique>) -> OrderedCliques {
    cliques
        .into_iter()
        .map(|clique| clique.into_iter().collect())
        .collect()
}

pub fn bron_kerbosch<VertexSet>(graph: &UndirectedGraph<VertexSet>) -> OrderedCliques
where
    VertexSet: VertexSetLike<VertexSet> + Send,
{
    let mut first: Option<OrderedCliques> = None;
    for func_index in 0..NUM_FUNCS {
        let mut reporter = SimpleReporter::new();
        explore(func_index, graph, &mut reporter);
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
    use graph::{Adjacencies, NewableUndirectedGraph};
    use reporter::Clique;
    use slimgraph::SlimUndirectedGraph;

    use std::collections::HashSet;

    fn bk_core<VertexSet>(adjacencies: &Vec<Vec<Vertex>>) -> OrderedCliques
    where
        VertexSet: VertexSetLike<VertexSet> + Send + Sync,
    {
        let adjacencies: Adjacencies<VertexSet> = adjacencies
            .iter()
            .map(|neighbours| neighbours.into_iter().cloned().collect())
            .collect();
        let graph = SlimUndirectedGraph::new(adjacencies);
        bron_kerbosch(&graph)
    }
    fn bk(adjacencies: Vec<Vec<Vertex>>, expected_cliques: Vec<Clique>) {
        let expected_cliques = order_cliques(expected_cliques);
        assert_eq!(bk_core::<BTreeSet<Vertex>>(&adjacencies), expected_cliques);
        assert_eq!(bk_core::<HashSet<Vertex>>(&adjacencies), expected_cliques);
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
    fn bk_order_5_penultimate() {
        bk(
            vec![
                vec![1, 2, 3, 4],
                vec![0, 2, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2],
                vec![0, 1, 2],
            ],
            vec![vec![0, 1, 2, 3], vec![0, 1, 2, 4]],
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
