extern crate bron_kerbosch;
extern crate random_graph;
extern crate stats;

extern crate csv;
extern crate rand;
extern crate rand_chacha;
extern crate structopt;

use bron_kerbosch::graph::UndirectedGraph;
use bron_kerbosch::reporter::SimpleReporter;
use bron_kerbosch::{order_cliques, OrderedCliques, FUNCS, NUM_FUNCS};
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

type Seconds = f32;
pub fn to_seconds(duration: Duration) -> Seconds {
    duration.as_secs() as Seconds + duration.subsec_nanos() as Seconds * 1e-9
}

pub fn generate_random_graph(rng: &mut impl Rng, order: Order, size: Size) -> UndirectedGraph {
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

pub fn bron_kerbosch_timed(
    graph: &UndirectedGraph,
    samples: u32,
    focus_on_func: Option<usize>,
) -> [SampleStatistics<Seconds>; NUM_FUNCS] {
    let mut times = [SampleStatistics::new(); NUM_FUNCS];
    let func_range = match focus_on_func {
        None => 0..NUM_FUNCS,
        Some(f) => (f - 1)..f,
    };
    let mut first: Option<OrderedCliques> = None;
    for _ in 0..samples {
        for func_index in func_range.clone() {
            let func = FUNCS[func_index];
            let sys_time = SystemTime::now();
            let mut reporter = SimpleReporter::new();
            func(&graph, &mut reporter);
            let mut diagnostic: Option<String> = None;
            let secs: Seconds = match sys_time.elapsed() {
                Ok(duration) => to_seconds(duration),
                Err(err) => {
                    diagnostic = Some(format!("Could not get time: {}", err));
                    -99.9
                }
            };
            if focus_on_func.is_none() {
                let current = order_cliques(reporter.cliques);
                match first.clone() {
                    None => {
                        first = Some(current);
                    }
                    Some(first_result) => {
                        if first_result != current {
                            diagnostic = Some(format!("oops, {:?} != {:?}", first_result, current));
                        }
                    }
                }
            }

            if let Some(d) = diagnostic {
                println!("Ver{}: {}", func_index + 1, d);
            } else {
                times[func_index].put(secs).unwrap();
            }
        }
    }
    times
}

fn bk(
    orderstr: &str,
    sizes: impl Iterator<Item = u32>,
    focus_on_func: Option<usize>,
) -> Result<(), std::io::Error> {
    const LANGUAGE: &str = "rust";
    const SAMPLES: u32 = 5;
    const SEED: [u8; 32] = [68u8; 32];

    let order = parse_positive_int(orderstr);
    {
        let mut writer: Option<csv::Writer<File>> = if focus_on_func.is_none() {
            let name = format!("bron_kerbosch_{}_order_{}", LANGUAGE, orderstr);
            let path = Path::join(Path::new(".."), Path::new(&name).with_extension("csv"));
            let file = File::create(path)?;
            let mut wtr = csv::Writer::from_writer(file);
            wtr.write_record(
                ["Size"]
                    .iter()
                    .map(|&s| String::from(s))
                    .chain((0..NUM_FUNCS).map(|i| format!("Ver{} min", i + 1)))
                    .chain((0..NUM_FUNCS).map(|i| format!("Ver{} max", i + 1)))
                    .chain((0..NUM_FUNCS).map(|i| format!("Ver{} mean", i + 1))),
            )?;
            Some(wtr)
        } else {
            None
        };
        for size in sizes {
            let mut rng = ChaChaRng::from_seed(SEED);
            let graph = generate_random_graph(&mut rng, Order::Of(order), Size::Of(size));
            let stats = bron_kerbosch_timed(&graph, SAMPLES, focus_on_func);
            for func_index in 0..NUM_FUNCS {
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

    if focus_on_func.is_none() {
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
    let opt = Opt::from_args();
    if opt.order.is_empty() {
        debug_assert!(false, "Run with --release for meaningful measurements");
        let sizes_50 = (750..=1000).step_by(5); // max 1225
        let sizes_10k = (1_000..10_000)
            .step_by(1_000)
            .chain((10_000..=200_000).step_by(10_000));
        bk("50", sizes_50, None)?;
        thread::sleep(Duration::from_secs(10));
        bk("10k", sizes_10k, None)?;
    } else if !opt.sizes.is_empty() {
        bk(
            &opt.order,
            opt.sizes.iter().map(|s| parse_positive_int(&s)),
            opt.focus,
        )?;
    } else {
        println!("Specify size(s) too")
    }
    Ok(())
}
