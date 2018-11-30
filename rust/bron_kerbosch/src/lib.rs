extern crate rand;

mod bron_kerbosch1;
mod bron_kerbosch2;
mod bron_kerbosch3;
mod bron_kerbosch4;
mod bron_kerbosch5;
mod bron_kerbosch6;
mod graph;
pub mod random_graph;
mod reporter;

use graph::UndirectedGraph;
use graph::Vertex;
use rand::Rng;
use random_graph::{new_undirected, Order, Size};
use reporter::Clique;
use reporter::{Reporter, SimpleReporter};
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::time::{Duration, SystemTime};

type OrderedClique = BTreeSet<Vertex>;
type OrderedCliques = BTreeSet<OrderedClique>;

pub const NUM_FUNCS: usize = 6;
static FUNCS: &'static [fn(
    graph: &UndirectedGraph,
    clique: Clique,
    candidates: HashSet<Vertex>,
    excluded: HashSet<Vertex>,
    reporter: &mut Reporter,
); NUM_FUNCS] = &[
    bron_kerbosch1::explore,
    bron_kerbosch2::explore,
    bron_kerbosch3::explore,
    bron_kerbosch4::explore,
    bron_kerbosch5::explore,
    bron_kerbosch6::explore,
];

fn order_cliques(cliques: Vec<Clique>) -> OrderedCliques {
    cliques
        .into_iter()
        .map(|clique| clique.into_iter().collect())
        .collect()
}

pub fn bron_kerbosch(graph: &UndirectedGraph) -> OrderedCliques {
    let mut first: Option<OrderedCliques> = None;
    for func in FUNCS {
        let mut candidates: HashSet<Vertex> = graph.connected_nodes();
        let mut excluded: HashSet<Vertex> = HashSet::new();
        let mut reporter = SimpleReporter::new();
        func(&graph, vec![], candidates, excluded, &mut reporter);
        let current = order_cliques(reporter.cliques);
        if first.is_none() {
            first = Some(current);
        } else {
            assert_eq!(current, *first.as_ref().unwrap());
        }
    }
    first.unwrap()
}

pub fn random_graph(rng: &mut impl Rng, order: Order, size: Size) -> UndirectedGraph {
    let Order::Of(order) = order;
    let Size::Of(size) = size;
    let sys_time = SystemTime::now();
    let graph = new_undirected(rng, Order::Of(order), Size::Of(size));
    let seconds = to_seconds(sys_time.elapsed().unwrap());
    println!(
        "random of order {}, size {}: (generating took {:.2}s)",
        order, size, seconds
    );
    graph
}

type Seconds = f32;
pub fn to_seconds(duration: Duration) -> Seconds {
    duration.as_secs() as Seconds + duration.subsec_nanos() as Seconds * 1e-9
}

pub fn bron_kerbosch_timed(graph: &UndirectedGraph) -> [Seconds; NUM_FUNCS] {
    const REPEATS: u32 = 10;
    let mut times = [-99.0; NUM_FUNCS];
    let mut first: Option<OrderedCliques> = None;
    for func_index in 0..FUNCS.len() {
        let func = FUNCS[func_index];
        let sys_time = SystemTime::now();
        let mut diagnostic: Option<String> = None;
        let mut reporter = SimpleReporter::new();
        for _ in 0..REPEATS {
            let mut candidates: HashSet<Vertex> = graph.connected_nodes();
            let mut excluded: HashSet<Vertex> = HashSet::new();
            reporter = SimpleReporter::new();
            func(&graph, vec![], candidates, excluded, &mut reporter);
            let current = order_cliques(reporter.cliques);
            match first.clone() {
                None => {
                    first = Some(current);
                }
                Some(first_result) => if first_result != current {
                    diagnostic = Some(format!("oops, {:?} != {:?}", first_result, current));
                },
            }
        }

        if diagnostic.is_none() {
            diagnostic = Some(match sys_time.elapsed() {
                Err(err) => format!("Could not get time: {}", err),
                Ok(duration) => {
                    times[func_index] = to_seconds(duration) / REPEATS as Seconds;
                    format!(
                        "{:5.2}s, {} recursive calls",
                        times[func_index], reporter.cnt
                    )
                }
            });
        }
        println!("Ver{}: {}", func_index + 1, diagnostic.unwrap());
    }
    times
}

#[cfg(test)]
mod tests {
    extern crate rand_chacha;

    use self::rand_chacha::ChaChaRng;
    use super::*;
    use rand::SeedableRng;
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
