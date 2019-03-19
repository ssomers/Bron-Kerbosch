extern crate bron_kerbosch;
extern crate stats;
mod random_graph;

extern crate csv;
extern crate rand;
extern crate rand_chacha;
extern crate structopt;

use bron_kerbosch::graph::{NewableUndirectedGraph, UndirectedGraph, Vertex, VertexSetLike};
use bron_kerbosch::reporter::SimpleReporter;
use bron_kerbosch::slimgraph::SlimUndirectedGraph;
use bron_kerbosch::{explore, order_cliques, OrderedCliques, FUNC_NAMES, NUM_FUNCS};
use random_graph::{new_undirected, Order, Size};
use stats::SampleStatistics;

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
    excluded_funcs: &Vec<usize>,
) -> [SampleStatistics<Seconds>; NUM_FUNCS]
where
    VertexSet: VertexSetLike + Send,
{
    let mut times: [SampleStatistics<Seconds>; NUM_FUNCS] = Default::default();
    let mut first: Option<OrderedCliques> = None;
    for sample in 0..samples {
        for func_index in 0..NUM_FUNCS {
            if excluded_funcs.contains(&func_index) {
                continue;
            }
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
            if secs >= 3.0 {
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
    excluded_funcs: &Vec<usize>,
    set_kind: &str,
) -> [SampleStatistics<Seconds>; NUM_FUNCS]
where
    VertexSet: VertexSetLike + Clone + Sync + Send,
{
    const SEED: [u8; 32] = [68u8; 32];

    if excluded_funcs.len() < NUM_FUNCS {
        let mut rng = ChaChaRng::from_seed(SEED);
        let graph: SlimUndirectedGraph<VertexSet> =
            generate_random_graph(&mut rng, set_kind, Order::Of(order), Size::Of(size));
        let stats = bron_kerbosch_timed(&graph, samples, excluded_funcs);
        for func_index in 0..NUM_FUNCS {
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
    excluded_funcs_btree: Vec<usize>,
    excluded_funcs_hash: Vec<usize>,
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
                    })),
            )?;
            Some(wtr)
        } else {
            None
        };
        for size in sizes {
            let stats1 = bk_core::<BTreeSet<Vertex>>(
                order,
                size,
                samples,
                &excluded_funcs_btree,
                "BTreeSet",
            );
            let stats2 =
                bk_core::<HashSet<Vertex>>(order, size, samples, &excluded_funcs_hash, "HashSet");
            if let Some(mut wtr) = writer.as_mut() {
                wtr.write_record(
                    [size]
                        .iter()
                        .map(|&i| i.to_string())
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
        let sizes_1m = (0..1_000_000)
            .step_by(250_000)
            .chain((1_000_000..=3_000_000).step_by(500_000));
        bk("100", 100, sizes_100.collect(), 5, vec![], vec![])?;
        thread::sleep(Duration::from_secs(10));
        bk("10k", 10_000, sizes_10k.collect(), 5, vec![], vec![])?;
        thread::sleep(Duration::from_secs(10));
        bk(
            "1M",
            1_000_000,
            sizes_1m.collect(),
            3,
            vec![7],
            vec![0, 1, 7],
        )?;
    } else if !opt.order.is_empty() && !opt.sizes.is_empty() {
        let order = parse_positive_int(&opt.order);
        let sizes = opt.sizes.iter().map(|s| parse_positive_int(&s));
        let mut excluded_funcs_btree = vec![];
        let mut excluded_funcs_hash = vec![];
        for i in 0..NUM_FUNCS {
            let out_of_focus = opt.ver.filter(|&f| f != i).is_some();
            if out_of_focus || opt.set.filter(|&k| k != 0).is_some() {
                excluded_funcs_btree.push(i);
            }
            if out_of_focus || opt.set.filter(|&k| k != 1).is_some() {
                excluded_funcs_hash.push(i);
            }
        }
        bk(
            &opt.order,
            order,
            sizes.collect(),
            1,
            excluded_funcs_btree,
            excluded_funcs_hash,
        )?;
    } else {
        eprintln!("Specify order and size(s)")
    }
    Ok(())
}
