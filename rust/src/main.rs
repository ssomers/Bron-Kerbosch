extern crate bron_kerbosch;
extern crate stats;
mod random_graph;

extern crate csv;
extern crate fnv;
extern crate rand;
extern crate rand_chacha;
extern crate structopt;

use bron_kerbosch::graph::{NewableUndirectedGraph, UndirectedGraph, Vertex, VertexSetLike};
use bron_kerbosch::reporter::SimpleReporter;
use bron_kerbosch::slimgraph::SlimUndirectedGraph;
use bron_kerbosch::{explore, order_cliques, OrderedCliques, FUNC_NAMES, NUM_FUNCS};
use random_graph::{new_undirected, Order, Size};
use stats::SampleStatistics;

use fnv::FnvHashSet;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(long = "ver")]
    ver: Option<usize>,

    #[structopt(long = "set")]
    set: Option<usize>,

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
fn to_seconds(duration: Duration) -> Seconds {
    duration.as_secs() as Seconds + duration.subsec_nanos() as Seconds * 1e-9
}

fn generate_random_graph<VertexSet, G>(
    rng: &mut impl Rng,
    set_kind: &str,
    order: Order,
    size: Size,
) -> G
where
    VertexSet: VertexSetLike + Clone,
    G: NewableUndirectedGraph<VertexSet>,
{
    let Order::Of(order) = order;
    let Size::Of(size) = size;
    let sys_time = SystemTime::now();
    let graph: G = new_undirected(rng, Order::Of(order), Size::Of(size));
    let seconds = to_seconds(sys_time.elapsed().unwrap());
    println!(
        "{}-based random graph of order {}, size {}: (generating took {:.2}s)",
        set_kind, order, size, seconds
    );
    graph
}

fn bron_kerbosch_timed<VertexSet>(
    graph: &UndirectedGraph<VertexSet>,
    samples: u32,
    func_indices: &Vec<usize>,
) -> [SampleStatistics<Seconds>; NUM_FUNCS]
where
    VertexSet: VertexSetLike + Send,
{
    let mut times: [SampleStatistics<Seconds>; NUM_FUNCS] = Default::default();
    let mut first: Option<OrderedCliques> = None;
    for sample in 0..samples {
        for &func_index in func_indices {
            let mut reporter = SimpleReporter::new();
            let sys_time = SystemTime::now();
            explore(func_index, graph, &mut reporter);
            let secs: Seconds = match sys_time.elapsed() {
                Ok(duration) => to_seconds(duration),
                Err(err) => {
                    eprintln!("Could not get time ({})", err);
                    -99.9
                }
            };
            if samples > 1 && secs >= 3.0 {
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
                            eprintln!(
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

fn bk_core<VertexSet>(
    order: u32,
    size: u32,
    samples: u32,
    func_indices: Vec<usize>,
    set_kind: &str,
) -> [SampleStatistics<Seconds>; NUM_FUNCS]
where
    VertexSet: VertexSetLike + Clone + Sync + Send,
{
    const SEED: [u8; 32] = [68u8; 32];

    if !func_indices.is_empty() {
        let mut rng = ChaChaRng::from_seed(SEED);
        let graph: SlimUndirectedGraph<VertexSet> =
            generate_random_graph(&mut rng, set_kind, Order::Of(order), Size::Of(size));
        let stats = bron_kerbosch_timed(&graph, samples, &func_indices);
        for func_index in func_indices {
            let name = FUNC_NAMES[func_index];
            let mean = stats[func_index].mean();
            let dev = stats[func_index].deviation();
            if !mean.is_nan() {
                println!("{:8}: {:5.2}s ±{:.0}%", name, mean, 100.0 * dev / mean);
            }
        }
        stats
    } else {
        let stats: [SampleStatistics<Seconds>; NUM_FUNCS] = Default::default();
        stats
    }
}

fn bk(
    orderstr: &str,
    order: u32,
    sizes: Vec<u32>,
    samples: u32,
    included_funcs: impl Fn(usize, u32) -> Vec<usize>,
) -> Result<(), std::io::Error> {
    const LANGUAGE: &str = "rust";

    let published = sizes.len() > 1;
    let name = format!("bron_kerbosch_{}_order_{}", LANGUAGE, orderstr);
    let temppath = Path::new("tmp").with_extension("csv");
    {
        let mut writer: Option<csv::Writer<File>> = if published {
            let file = File::create(&temppath)?;
            let mut wtr = csv::Writer::from_writer(file);
            wtr.write_record(
                ["Size"]
                    .iter()
                    .map(|&s| String::from(s))
                    .chain(FUNC_NAMES.iter().flat_map(|name| {
                        vec![
                            format!("{}@BTreeSet min", name),
                            format!("{}@BTreeSet mean", name),
                            format!("{}@BTreeSet max", name),
                        ]
                    }))
                    .chain(FUNC_NAMES.iter().flat_map(|name| {
                        vec![
                            format!("{}@HashSet min", name),
                            format!("{}@HashSet mean", name),
                            format!("{}@HashSet max", name),
                        ]
                    }))
                    .chain(FUNC_NAMES.iter().flat_map(|name| {
                        vec![
                            format!("{}@FnvHashSet min", name),
                            format!("{}@FnvHashSet mean", name),
                            format!("{}@FnvHashSet max", name),
                        ]
                    })),
            )?;
            Some(wtr)
        } else {
            None
        };
        for size in sizes {
            let stats0 = bk_core::<BTreeSet<Vertex>>(
                order,
                size,
                samples,
                included_funcs(0, size),
                "BTreeSet",
            );
            let stats1 = bk_core::<HashSet<Vertex>>(
                order,
                size,
                samples,
                included_funcs(1, size),
                "HashSet",
            );
            let stats2 = bk_core::<FnvHashSet<Vertex>>(
                order,
                size,
                samples,
                included_funcs(2, size),
                "FnvHashSet",
            );
            if let Some(mut wtr) = writer.as_mut() {
                wtr.write_record(
                    [size]
                        .iter()
                        .map(|&i| i.to_string())
                        .chain(stats0.into_iter().flat_map(|s| {
                            vec![
                                s.min().to_string(),
                                s.mean().to_string(),
                                s.max().to_string(),
                            ]
                        }))
                        .chain(stats1.into_iter().flat_map(|s| {
                            vec![
                                s.min().to_string(),
                                s.mean().to_string(),
                                s.max().to_string(),
                            ]
                        }))
                        .chain(stats2.into_iter().flat_map(|s| {
                            vec![
                                s.min().to_string(),
                                s.mean().to_string(),
                                s.max().to_string(),
                            ]
                        })),
                )?;
            }
        }
    }

    if published {
        let path = Path::join(Path::new(".."), Path::new(&name).with_extension("csv"));
        std::fs::rename(temppath, path)?;
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
    if opt.order.is_empty() && opt.ver.is_none() && opt.set.is_none() {
        debug_assert!(false, "Run with --release for meaningful measurements");
        let sizes_100 = (2_000..=3_000).step_by(50); // max 4_950
        let sizes_10k = (1_000..10_000)
            .step_by(1_000)
            .chain((10_000..=200_000).step_by(10_000)); // max 499_500
        let sizes_1m = (0..25_000)
            .step_by(2_500)
            .chain((25_000..250_000).step_by(25_000))
            .chain((250_000..1_000_000).step_by(250_000))
            .chain((1_000_000..=5_000_000).step_by(1_000_000));
        let all_funcs = |_kind: usize, _size: u32| -> Vec<usize> { (0..NUM_FUNCS).collect() };
        bk("100", 100, sizes_100.collect(), 5, all_funcs)?;
        thread::sleep(Duration::from_secs(7));
        bk("10k", 10_000, sizes_10k.collect(), 5, all_funcs)?;
        thread::sleep(Duration::from_secs(7));
        bk(
            "1M",
            1_000_000,
            sizes_1m.collect(),
            3,
            |kind: usize, size: u32| -> Vec<usize> {
                (0..NUM_FUNCS)
                    .filter(|func_index| {
                        // No need to keep testing the unimproved ones
                        match func_index {
                            9 => true,
                            0 | 2 | 7 => false,
                            1 => kind == 0 || size <= 25_000,
                            _ => kind == 0 && size <= 500_000,
                        }
                    })
                    .collect()
            },
        )?;
    } else if !opt.order.is_empty() && !opt.sizes.is_empty() {
        let order = parse_positive_int(&opt.order);
        let sizes = opt.sizes.iter().map(|s| parse_positive_int(&s));
        let included_funcs = |kind: usize, _size: u32| -> Vec<usize> {
            if opt.set.filter(|&k| k != kind).is_some() {
                vec![]
            } else if let Some(func_index) = opt.ver {
                vec![func_index]
            } else {
                (0..NUM_FUNCS).collect()
            }
        };
        bk(&opt.order, order, sizes.collect(), 1, included_funcs)?;
    } else {
        eprintln!("Specify order and size(s)")
    }
    Ok(())
}
