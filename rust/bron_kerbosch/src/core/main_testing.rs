use crate::VertexSetLike;

use super::clique_ordering::OrderedCliques;
use super::graph::Graph;
use super::vertex::Vertex;

pub struct TestGraph<VertexSet: VertexSetLike> {
    pub name: &'static str,
    pub graph: Graph<VertexSet>,
    pub cliques: OrderedCliques,
}

struct TestData {
    name: &'static str,
    adjacencies: Vec<Vec<usize>>,
    cliques: Vec<Vec<usize>>,
}

fn verticise<VertexSet: VertexSetLike>(vertex_indices: &[usize]) -> VertexSet {
    vertex_indices.iter().copied().map(Vertex::new).collect()
}

pub fn all_test_graphs<VertexSet: VertexSetLike>() -> Vec<TestGraph<VertexSet>> {
    vec![
        TestData {
            name: "order_0",
            adjacencies: vec![],
            cliques: vec![],
        },
        TestData {
            name: "order_1",
            adjacencies: vec![vec![]],
            cliques: vec![],
        },
        TestData {
            name: "order_2_isolated",
            adjacencies: vec![vec![], vec![]],
            cliques: vec![],
        },
        TestData {
            name: "2_connected",
            adjacencies: vec![vec![1], vec![0]],
            cliques: vec![vec![0, 1]],
        },
        TestData {
            name: "order_3_size_1_left",
            adjacencies: vec![vec![1], vec![0], vec![]],
            cliques: vec![vec![0, 1]],
        },
        TestData {
            name: "order_3_size_1_long",
            adjacencies: vec![vec![2], vec![], vec![0]],
            cliques: vec![vec![0, 2]],
        },
        TestData {
            name: "order_3_size_1_right",
            adjacencies: vec![vec![], vec![2], vec![1]],
            cliques: vec![vec![1, 2]],
        },
        TestData {
            name: "order_3_size_2",
            adjacencies: vec![vec![1], vec![0, 2], vec![1]],
            cliques: vec![vec![0, 1], vec![1, 2]],
        },
        TestData {
            name: "order_3_size_3",
            adjacencies: vec![vec![1, 2], vec![0, 2], vec![0, 1]],
            cliques: vec![vec![0, 1, 2]],
        },
        TestData {
            name: "order_4_size_2",
            adjacencies: vec![vec![1], vec![0], vec![3], vec![2]],
            cliques: vec![vec![0, 1], vec![2, 3]],
        },
        TestData {
            name: "order_4_size_3_bus",
            adjacencies: vec![vec![1], vec![0, 2], vec![1, 3], vec![2]],
            cliques: vec![vec![0, 1], vec![1, 2], vec![2, 3]],
        },
        TestData {
            name: "order_4_size_3_star",
            adjacencies: vec![vec![1, 2, 3], vec![0], vec![0], vec![0]],
            cliques: vec![vec![0, 1], vec![0, 2], vec![0, 3]],
        },
        TestData {
            name: "order_4_size_4_p",
            adjacencies: vec![vec![1], vec![0, 2, 3], vec![1, 3], vec![1, 2]],
            cliques: vec![vec![0, 1], vec![1, 2, 3]],
        },
        TestData {
            name: "order_4_size_4_square",
            adjacencies: vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]],
            cliques: vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![2, 3]],
        },
        TestData {
            name: "order_4_size_5",
            adjacencies: vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]],
            cliques: vec![vec![0, 1, 2], vec![0, 2, 3]],
        },
        TestData {
            name: "order_4_size_6",
            adjacencies: vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]],
            cliques: vec![vec![0, 1, 2, 3]],
        },
        TestData {
            name: "order_5_penultimate",
            adjacencies: vec![
                vec![1, 2, 3, 4],
                vec![0, 2, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2],
                vec![0, 1, 2],
            ],
            cliques: vec![vec![0, 1, 2, 3], vec![0, 1, 2, 4]],
        },
        TestData {
            name: "sample",
            adjacencies: vec![
                vec![],
                vec![2, 3, 4],
                vec![1, 3, 4, 5],
                vec![1, 2, 4, 5],
                vec![1, 2, 3],
                vec![2, 3, 6, 7],
                vec![5, 7],
                vec![5, 6],
            ],
            cliques: vec![vec![1, 2, 3, 4], vec![2, 3, 5], vec![5, 6, 7]],
        },
        TestData {
            name: "bigger",
            adjacencies: vec![
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
            cliques: vec![
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
        },
    ]
    .iter()
    .map(|r| TestGraph {
        name: r.name,
        graph: Graph::new(
            r.adjacencies
                .iter()
                .map(|neighbours| verticise(neighbours))
                .collect(),
        ),
        cliques: r.cliques.iter().map(|clique| verticise(clique)).collect(),
    })
    .collect()
}
