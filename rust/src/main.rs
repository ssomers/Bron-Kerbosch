extern crate bron_kerbosch;
extern crate random_graph;
extern crate stats;

extern crate csv;
extern crate rand;
extern crate rand_chacha;
extern crate structopt;

use bron_kerbosch::graph::{NewableUndirectedGraph, UndirectedGraph};
use bron_kerbosch::reporter::SimpleReporter;
use bron_kerbosch::slimgraph::SlimUndirectedGraph;
use bron_kerbosch::{order_cliques, OrderedCliques, FUNCS, FUNC_NAMES, NUM_FUNCS};
use random_graph::{new_undirected, Order, Size};
use stats::SampleStatistics;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "f", long = "focus")]
    focus: Option<usize>,

    #[structopt(name = "order", default_value = "")]
    order: String,

    #[structopt(name = "sizes")]
    sizes: Vec<String>,
}

fn parse_positive_int(value: &str) -> u32 {
    let numstr: &str;
    let factor: u32;
    if value.ends_with("M") {
        numstr = &value[0..value.len() - 1];
        factor = 1_000_000;
    } else if value.ends_with("k") {
        numstr = &value[0..value.len() - 1];
        factor = 1_000;
    } else {
        numstr = value;
        factor = 1;
    }
    let num: u32 = numstr
        .parse()
        .expect(&format!("{} is not a positive integer", numstr));
    num * factor
}

type Seconds = f32;
pub fn to_seconds(duration: Duration) -> Seconds {
    duration.as_secs() as Seconds + duration.subsec_nanos() as Seconds * 1e-9
}

pub fn generate_random_graph<G: NewableUndirectedGraph>(
    rng: &mut impl Rng,
    order: Order,
    size: Size,
) -> G {
    let Order::Of(order) = order;
    let Size::Of(size) = size;
    let sys_time = SystemTime::now();
    let graph: G = new_undirected(rng, Order::Of(order), Size::Of(size));
    let seconds = to_seconds(sys_time.elapsed().unwrap());
    println!(
        "random of order {}, size {}: (generating took {:.2}s)",
        order, size, seconds
    );
    graph
}

pub fn bron_kerbosch_timed(
    graph: &UndirectedGraph,
    samples: u32,
    func_indices: &Vec<usize>,
) -> [SampleStatistics<Seconds>; NUM_FUNCS] {
    let mut times = [SampleStatistics::new(); NUM_FUNCS];
    let mut first: Option<OrderedCliques> = None;
    for sample in 0..samples {
        for &func_index in func_indices {
            let func = FUNCS[func_index];
            let mut reporter = SimpleReporter::new();
            let sys_time = SystemTime::now();
            func(graph, &mut reporter);
            let secs: Seconds = match sys_time.elapsed() {
                Ok(duration) => to_seconds(duration),
                Err(err) => {
                    println!(
                        "  {:8}: Could not get time ({})",
                        FUNC_NAMES[func_index], err
                    );
                    -99.9
                }
            };
            if secs >= 1.0 {
                println!("  {:8}: {:5.2}s", FUNC_NAMES[func_index], secs);
            }
            if sample < 2 {
                let current = order_cliques(reporter.cliques);
                match first.clone() {
                    None => {
                        first = Some(current);
                    }
                    Some(first_result) => {
                        if first_result != current {
                            println!(
                                "  {:8}: expected {} cliques, obtained {} different cliques",
                                FUNC_NAMES[func_index],
                                first_result.len(),
                                current.len()
                            );
                        }
                    }
                }
            }

            times[func_index].put(secs).unwrap();
        }
    }
    times
}

fn bk(
    orderstr: &str,
    order: u32,
    sizes: Vec<u32>,
    samples: u32,
    func_indices: &Vec<usize>,
) -> Result<(), std::io::Error> {
    const LANGUAGE: &str = "rust";
    const SEED: [u8; 32] = [68u8; 32];

    let published = func_indices.len() > 1 && sizes.len() > 1;
    {
        let mut writer: Option<csv::Writer<File>> = if published {
            let name = format!("bron_kerbosch_{}_order_{}", LANGUAGE, orderstr);
            let path = Path::join(Path::new(".."), Path::new(&name).with_extension("csv"));
            let file = File::create(path)?;
            let mut wtr = csv::Writer::from_writer(file);
            wtr.write_record(
                ["Size"]
                    .iter()
                    .map(|&s| String::from(s))
                    .chain((0..NUM_FUNCS).map(|i| format!("{} min", FUNC_NAMES[i])))
                    .chain((0..NUM_FUNCS).map(|i| format!("{} max", FUNC_NAMES[i])))
                    .chain((0..NUM_FUNCS).map(|i| format!("{} mean", FUNC_NAMES[i]))),
            )?;
            Some(wtr)
        } else {
            None
        };
        for size in sizes {
            let mut rng = ChaChaRng::from_seed(SEED);
            let graph: SlimUndirectedGraph =
                generate_random_graph(&mut rng, Order::Of(order), Size::Of(size));
            let stats = bron_kerbosch_timed(&graph, samples, &func_indices);
            for func_index in 0..NUM_FUNCS {
                let mean = stats[func_index].mean();
                let dev = stats[func_index].deviation();
                println!(
                    "{:8}: {:5.2}s {}{:5.2}",
                    FUNC_NAMES[func_index], mean, 177 as char, dev
                );
            }
            if let Some(mut wtr) = writer.as_mut() {
                wtr.write_record(
                    [size]
                        .iter()
                        .map(|&i| i.to_string())
                        .chain(stats.iter().map(|&s| s.min().to_string()))
                        .chain(stats.iter().map(|&s| s.max().to_string()))
                        .chain(stats.iter().map(|&s| s.mean().to_string())),
                )?;
            }
        }
    }

    if published {
        let publish = Path::new("..")
            .join(Path::new("python3"))
            .join(Path::new("publish.py"));
        let rc = std::process::Command::new("python")
            .arg(publish)
            .arg(LANGUAGE)
            .arg(orderstr)
            .status()?;
        assert!(rc.success());
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let all_func_indices: Vec<usize> = (0..NUM_FUNCS).collect();
    let fast_func_indices: Vec<usize> = vec![2, 3, 4, 5, 6, 8, 9];
    let opt = Opt::from_args();
    if opt.order.is_empty() {
        debug_assert!(false, "Run with --release for meaningful measurements");
        let sizes_100 = (2_000..=3_000).step_by(50); // max 4_950
        let sizes_10k = (1_000..10_000)
            .step_by(1_000)
            .chain((10_000..=200_000).step_by(10_000)); // max 499_500
        let sizes_1m = (0..2_000_000).step_by(250_000);
        bk("100", 100, sizes_100.collect(), 5, &all_func_indices)?;
        thread::sleep(Duration::from_secs(10));
        bk("10k", 10_000, sizes_10k.collect(), 5, &all_func_indices)?;
        thread::sleep(Duration::from_secs(10));
        bk("1M", 1_000_000, sizes_1m.collect(), 3, &fast_func_indices)?;
    } else if !opt.sizes.is_empty() {
        let order = parse_positive_int(&opt.order);
        let sizes = opt.sizes.iter().map(|s| parse_positive_int(&s));
        let func_indices = match opt.focus {
            None => all_func_indices,
            Some(f) => vec![f],
        };
        bk(&opt.order, order, sizes.collect(), 1, &func_indices)?;
    } else {
        println!("Specify size(s) too")
    }
    Ok(())
}
