extern crate bron_kerbosch;
extern crate rand;
extern crate rand_chacha;
extern crate structopt;

use bron_kerbosch::random_graph::{Order, Size};
use bron_kerbosch::{bron_kerbosch_timed, random_graph};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(name = "order", default_value = "0")]
    order: u32,

    #[structopt(name = "size")]
    size: Vec<u32>,
}

fn main() {
    let opt = Opt::from_args();
    let mut sizes_by_order: HashMap<u32, Vec<u32>> = HashMap::new();
    if opt.order > 0 {
        sizes_by_order.insert(opt.order, opt.size);
    } else {
        debug_assert!(false, "Run with --release for meaningful measurements");
        let sizes_50 = (750..1_000).step_by(10); // max 1225
        let sizes_10k = (1_000..10_000)
            .step_by(1_000)
            .chain((10_000..100_000).step_by(10_000));
        sizes_by_order.insert(50, sizes_50.collect());
        sizes_by_order.insert(10_000, sizes_10k.collect());
    }

    for (order, sizes) in sizes_by_order {
        let mut times_per_size: Vec<Vec<f32>> = Vec::new();
        for size in &sizes {
            let mut rng = ChaChaRng::from_seed([68u8; 32]);
            let graph = random_graph(&mut rng, Order::Of(order), Size::Of(*size));
            times_per_size.push(bron_kerbosch_timed(&graph));
        }

        let rc = std::process::Command::new("python")
            .arg("..\\python3\\publish.py")
            .arg("rust")
            .arg(times_per_size[0].len().to_string())
            .arg(order.to_string())
            .arg(sizes.len().to_string())
            .args(sizes.iter().map(u32::to_string))
            .args(
                times_per_size
                    .iter()
                    .flat_map(|times| times.iter().map(f32::to_string)),
            ).status()
            .unwrap();
        assert!(rc.success());
    }
}
