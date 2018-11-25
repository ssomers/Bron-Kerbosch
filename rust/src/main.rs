extern crate bron_kerbosch;
extern crate rand;

use bron_kerbosch::bron_kerbosch;
use bron_kerbosch::random_graph::{new_undirected, Order, Size};

fn main() {
    let mut rng = rand::thread_rng();
    let graph = new_undirected(&mut rng, Order::Of(4), Size::Of(5));
    println!("{:?}", bron_kerbosch(&graph));
}
