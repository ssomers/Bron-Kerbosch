mod bron_kerbosch1a;
mod bron_kerbosch1b;
mod bron_kerbosch2a_gp;
mod bron_kerbosch2b;
mod bron_kerbosch2b_gp;
mod bron_kerbosch2b_gpx;
mod bron_kerbosch2b_rp;
mod bron_kerbosch3_gp;
mod bron_kerbosch3_gpx;
mod bron_kerbosch3_mt;
mod bron_kerbosch_pivot;
pub mod graph;
pub mod graph_degeneracy;
pub mod pile;
pub mod reporter;
pub mod reporters;
pub mod slimgraph;
mod vertex;
mod vertexsetlike;
mod vertexsetlikes;

use graph::{UndirectedGraph, Vertex, VertexSetLike};
use reporter::{Clique, Reporter};
use reporters::SimpleReporter;
use std::collections::BTreeSet;

#[cfg(not(miri))]
pub const NUM_FUNCS: usize = 10;
#[cfg(miri)]
pub const NUM_FUNCS: usize = 9;
pub static FUNC_NAMES: &[&str; 10] = &[
    "Ver1",
    "Ver1½",
    "Ver2-GP",
    "Ver2½",
    "Ver2½-GP",
    "Ver2½-GPX",
    "Ver2½-RP",
    "Ver3½-GP",
    "Ver3½-GPX",
    "Ver3½=GPc",
];

pub fn explore<Graph, Rprtr>(func_index: usize, graph: &Graph, reporter: &mut Rprtr)
where
    Graph: UndirectedGraph,
    Graph::VertexSet: VertexSetLike + Send,
    Rprtr: Reporter,
{
    match func_index {
        0 => bron_kerbosch1a::explore(graph, reporter),
        1 => bron_kerbosch1b::explore(graph, reporter),
        2 => bron_kerbosch2a_gp::explore(graph, reporter),
        3 => bron_kerbosch2b::explore(graph, reporter),
        4 => bron_kerbosch2b_gp::explore(graph, reporter),
        5 => bron_kerbosch2b_gpx::explore(graph, reporter),
        6 => bron_kerbosch2b_rp::explore(graph, reporter),
        7 => bron_kerbosch3_gp::explore(graph, reporter),
        8 => bron_kerbosch3_gpx::explore(graph, reporter),
        9 => bron_kerbosch3_mt::explore(graph, reporter),
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

pub fn bron_kerbosch<Graph>(graph: &Graph) -> OrderedCliques
where
    Graph: UndirectedGraph,
    Graph::VertexSet: VertexSetLike + Send,
{
    let mut first: Option<OrderedCliques> = None;
    for func_index in 0..NUM_FUNCS {
        let mut reporter = SimpleReporter::default();
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
    use graph::{NewableUndirectedGraph, VertexMap};
    use slimgraph::SlimUndirectedGraph;

    use fnv::FnvHashSet;
    use hashbrown;
    use std::collections::HashSet;

    fn bk_core<VertexSet>(adjacencies: &Vec<Vec<Vertex>>) -> OrderedCliques
    where
        VertexSet: VertexSetLike + Send + Sync,
    {
        let adjacencies: Vec<VertexSet> = adjacencies
            .iter()
            .map(|neighbours| neighbours.into_iter().copied().collect())
            .collect();
        let adjacencies = VertexMap::sneak_in(adjacencies);
        let graph = SlimUndirectedGraph::new(adjacencies);
        bron_kerbosch(&graph)
    }

    fn bk(adjacencies: Vec<Vec<usize>>, expected_cliques: Vec<Vec<usize>>) {
        let adjacencies = adjacencies
            .into_iter()
            .map(|vertices| vertices.into_iter().map(|v| Vertex::new(v)).collect())
            .collect();
        let expected_cliques = order_cliques(
            expected_cliques
                .into_iter()
                .map(|clique| clique.into_iter().map(|v| Vertex::new(v)).collect())
                .collect(),
        );
        assert_eq!(bk_core::<BTreeSet<Vertex>>(&adjacencies), expected_cliques);
        assert_eq!(bk_core::<HashSet<Vertex>>(&adjacencies), expected_cliques);
        assert_eq!(
            bk_core::<FnvHashSet<Vertex>>(&adjacencies),
            expected_cliques
        );
        assert_eq!(
            bk_core::<hashbrown::HashSet<Vertex>>(&adjacencies),
            expected_cliques
        );
        assert_eq!(bk_core::<Vec<Vertex>>(&adjacencies), expected_cliques);
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
    fn bk_order_3_size_1_left() {
        bk(vec![vec![1], vec![0], vec![]], vec![vec![0, 1]]);
    }

    #[test]
    fn bk_order_3_size_1_long() {
        bk(vec![vec![2], vec![], vec![0]], vec![vec![0, 2]]);
    }

    #[test]
    fn bk_order_3_size_1_right() {
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
    fn bk_order_4_size_2() {
        bk(
            vec![vec![1], vec![0], vec![3], vec![2]],
            vec![vec![0, 1], vec![2, 3]],
        );
    }

    #[test]
    fn bk_order_4_size_3_bus() {
        bk(
            vec![vec![1], vec![0, 2], vec![1, 3], vec![2]],
            vec![vec![0, 1], vec![1, 2], vec![2, 3]],
        );
    }

    #[test]
    fn bk_order_4_size_3_star() {
        bk(
            vec![vec![1, 2, 3], vec![0], vec![0], vec![0]],
            vec![vec![0, 1], vec![0, 2], vec![0, 3]],
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

    #[test]
    fn bk_bigger() {
        bk(
            vec![
                vec![1, 2, 3, 4, 6, 7],
                vec![0, 3, 6, 7, 8, 9],
                vec![0, 3, 5, 7, 8, 9],
                vec![0, 1, 2, 4, 9],
                vec![0, 3, 6, 7, 9],
                vec![2, 6],
                vec![0, 1, 4, 5, 9],
                vec![0, 1, 2, 4, 9],
                vec![1, 2],
                vec![1, 2, 3, 4, 6, 7],
            ],
            vec![
                vec![0, 1, 3],
                vec![0, 1, 6],
                vec![0, 1, 7],
                vec![0, 2, 3],
                vec![0, 2, 7],
                vec![0, 3, 4],
                vec![0, 4, 6],
                vec![0, 4, 7],
                vec![1, 3, 9],
                vec![1, 6, 9],
                vec![1, 7, 9],
                vec![1, 8],
                vec![2, 3, 9],
                vec![2, 5],
                vec![2, 7, 9],
                vec![2, 8],
                vec![3, 4, 9],
                vec![4, 6, 9],
                vec![4, 7, 9],
                vec![5, 6],
            ],
        );
    }
}
