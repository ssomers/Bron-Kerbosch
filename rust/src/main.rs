extern crate bron_kerbosch;
extern crate rand;

use bron_kerbosch::bron_kerbosch;
use bron_kerbosch::random_graph::{new_undirected, Order, Size};
use bron_kerbosch::reporter::SimpleReporter;

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let graph = new_undirected(&mut rng, Order::Of(4), Size::Of(5));
    let mut reporter = SimpleReporter::new();
    bron_kerbosch(&graph, &mut reporter);
    println!("{:?}", reporter.cliques);
}
