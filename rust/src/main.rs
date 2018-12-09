extern crate bron_kerbosch;
extern crate csv;
extern crate rand;
extern crate rand_chacha;
extern crate structopt;

use bron_kerbosch::random_graph::{Order, Size};
use bron_kerbosch::{bron_kerbosch_timed, random_graph, NUM_FUNCS};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(name = "order", default_value = "")]
    order: String,

    #[structopt(name = "sizes")]
    sizes: Vec<u32>,
}

fn parse_positive_int(value: &str) -> u32 {
    let numstr: &str;
    let factor: u32;
    if value.ends_with("k") {
        numstr = &value[0..value.len() - 1];
        factor = 1000;
    } else {
        numstr = value;
        factor = 1;
    }
    let num: u32 = numstr
        .parse()
        .expect(&format!("{} is not a positive integer", numstr));
    num * factor
}

fn bk<I>(orderstr: &str, sizes: I) -> Result<(), std::io::Error>
where
    I: Iterator<Item = u32>,
{
    const LANGUAGE: &str = "rust";
    const SAMPLES: u32 = 10;
    const SEED: [u8; 32] = [68u8; 32];
    {
        let order = parse_positive_int(orderstr);
        let name = format!("bron_kerbosch_{}_order_{}", LANGUAGE, orderstr);
        let path = Path::join(Path::new(".."), Path::new(&name).with_extension("csv"));
        let file = File::create(path)?;
        let mut wtr = csv::Writer::from_writer(file);
        wtr.write_record(
            ["Size"]
                .iter()
                .map(|&s| String::from(s))
                .chain((0..NUM_FUNCS).map(|i| format!("Ver{} seconds", i + 1)))
                .chain((0..NUM_FUNCS).map(|i| format!("Ver{} error", i + 1))),
        )?;
        for size in sizes {
            let mut rng = ChaChaRng::from_seed(SEED);
            let graph = random_graph(&mut rng, Order::Of(order), Size::Of(size));
            let stats = bron_kerbosch_timed(&graph, SAMPLES);
            assert_eq!(stats.len(), NUM_FUNCS as usize);
            for func_index in 0..NUM_FUNCS {
                assert!(stats[func_index].is_populated());
                let mean = stats[func_index].mean();
                let pct = stats[func_index].deviation() / mean * 100.0;
                println!(
                    "Ver{}: {:5.2}s {}{:.0}%",
                    func_index + 1,
                    mean,
                    177 as char,
                    pct
                );
            }
            let times = stats.iter().map(|s| s.mean().to_string());
            let errors = stats.iter().map(|s| s.deviation().to_string());
            wtr.write_record(
                [size]
                    .iter()
                    .map(|&i| i.to_string())
                    .chain(times)
                    .chain(errors),
            )?;
        }
    }

    let publish = Path::new("..")
        .join(Path::new("python3"))
        .join(Path::new("publish.py"));
    let rc = std::process::Command::new("python")
        .arg(publish)
        .arg(LANGUAGE)
        .arg(orderstr)
        .status()?;
    assert!(rc.success());
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();
    if !opt.order.is_empty() {
        bk(&opt.order, opt.sizes.iter().cloned())?;
    } else {
        debug_assert!(false, "Run with --release for meaningful measurements");
        let sizes_50 = (600..=900).step_by(10); // max 1225
        let sizes_10k = (1_000..10_000)
            .step_by(1_000)
            .chain((10_000..100_000).step_by(10_000));
        bk("50", sizes_50)?;
        thread::sleep(Duration::from_secs(10));
        bk("10k", sizes_10k)?;
    }
    Ok(())
}
