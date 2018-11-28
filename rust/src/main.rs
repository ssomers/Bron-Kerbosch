extern crate bron_kerbosch;
extern crate csv;
extern crate rand;
extern crate rand_chacha;
extern crate structopt;

use bron_kerbosch::random_graph::{Order, Size};
use bron_kerbosch::{bron_kerbosch_timed, random_graph};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use std::fs::File;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(name = "order", default_value = "0")]
    order: u32,

    #[structopt(name = "size")]
    size: Vec<u32>,
}

fn bk<I>(order: u32, sizes: I) -> Result<(), std::io::Error>
where
    I: Iterator<Item = u32>,
{
    const LANGUAGE: &str = "rust";
    const NUM_FUNCS: u32 = 2;
    {
        let name = format!("bron_kerbosch_{}_order_{}", LANGUAGE, order);
        let path = Path::join(Path::new(".."), Path::new(&name).with_extension("csv"));
        let file = File::create(path)?;
        let mut wtr = csv::Writer::from_writer(file);
        wtr.write_field("Size")?;
        wtr.write_record((0..NUM_FUNCS).map(|i| format!("Ver{}", i + 1)))?;
        for size in sizes {
            let mut rng = ChaChaRng::from_seed([68u8; 32]);
            let graph = random_graph(&mut rng, Order::Of(order), Size::Of(size));
            let times = bron_kerbosch_timed(&graph);
            assert_eq!(times.len(), NUM_FUNCS as usize);
            wtr.write_field(size.to_string())?;
            wtr.write_record(times.iter().map(f32::to_string))?;
        }
    }

    let rc = std::process::Command::new("python")
        .arg(
            Path::new("..")
                .join(Path::new("python3"))
                .join(Path::new("publish.py")),
        ).arg(LANGUAGE)
        .arg(order.to_string())
        .status()?;
    assert!(rc.success());
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if opt.order > 0 {
        bk(opt.order, opt.size.iter().cloned()).unwrap();
    } else {
        debug_assert!(false, "Run with --release for meaningful measurements");
        let sizes_50 = (750..1_000).step_by(10); // max 1225
        let sizes_10k = (1_000..10_000)
            .step_by(1_000)
            .chain((10_000..100_000).step_by(10_000));
        bk(50, sizes_50).unwrap();
        bk(10_000, sizes_10k).unwrap();
    }
}
