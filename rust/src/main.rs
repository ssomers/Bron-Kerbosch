extern crate bron_kerbosch;
extern crate stats;
mod random_graph;

extern crate csv;
extern crate fnv;
extern crate hashbrown;
extern crate itertools;
extern crate rand;
extern crate rand_chacha;
extern crate structopt;
extern crate strum;
extern crate strum_macros;

use bron_kerbosch::graph::{NewableUndirectedGraph, UndirectedGraph, Vertex, VertexSetLike};
use bron_kerbosch::reporter::SimpleReporter;
use bron_kerbosch::slimgraph::SlimUndirectedGraph;
use bron_kerbosch::{explore, order_cliques, OrderedCliques, FUNC_NAMES, NUM_FUNCS};
use random_graph::{new_undirected, Order, Size};
use stats::SampleStatistics;

use fnv::FnvHashSet;
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};
use structopt::StructOpt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(long = "ver")]
    ver: Option<usize>,

    #[structopt(long = "set")]
    set: Option<SetType>,

    #[structopt(name = "order", default_value = "")]
    order: String,

    #[structopt(name = "sizes")]
    sizes: Vec<String>,
}

#[derive(Copy, Clone, Debug, Display, EnumIter, EnumString, Eq, PartialEq, Ord, PartialOrd)]
enum SetType {
    BTreeSet,
    HashSet,
    #[strum(to_string = "fnv")]
    Fnv,
    #[strum(to_string = "hashbrown")]
    Hashbrown,
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
    set_type: SetType,
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
        set_type, order, size, seconds
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

fn bk_core_core<VertexSet>(
    order: u32,
    size: u32,
    samples: u32,
    set_type: SetType,
    func_indices: &Vec<usize>,
) -> [SampleStatistics<Seconds>; NUM_FUNCS]
where
    VertexSet: VertexSetLike + Clone + Sync + Send,
{
    const SEED: [u8; 32] = [68u8; 32];

    let mut rng = ChaChaRng::from_seed(SEED);
    let graph: SlimUndirectedGraph<VertexSet> =
        generate_random_graph(&mut rng, set_type, Order::Of(order), Size::Of(size));
    bron_kerbosch_timed(&graph, samples, func_indices)
}

fn bk_core(
    order: u32,
    size: u32,
    samples: u32,
    included_funcs: &impl Fn(SetType, u32) -> Vec<usize>,
    set_type: SetType,
) -> [SampleStatistics<Seconds>; NUM_FUNCS] {
    let func_indices = included_funcs(set_type, size);
    if func_indices.is_empty() {
        let stats: [SampleStatistics<Seconds>; NUM_FUNCS] = Default::default();
        stats
    } else {
        let stats = match set_type {
            SetType::BTreeSet => {
                bk_core_core::<BTreeSet<Vertex>>(order, size, samples, set_type, &func_indices)
            }
            SetType::HashSet => {
                bk_core_core::<HashSet<Vertex>>(order, size, samples, set_type, &func_indices)
            }
            SetType::Fnv => {
                bk_core_core::<FnvHashSet<Vertex>>(order, size, samples, set_type, &func_indices)
            }
            SetType::Hashbrown => bk_core_core::<hashbrown::HashSet<Vertex>>(
                order,
                size,
                samples,
                set_type,
                &func_indices,
            ),
        };
        for func_index in func_indices {
            let name = FUNC_NAMES[func_index];
            let mean = stats[func_index].mean();
            let dev = stats[func_index].deviation();
            if !mean.is_nan() {
                println!("{:8}: {:5.2}s ±{:.0}%", name, mean, 100.0 * dev / mean);
            }
        }
        stats
    }
}

fn bk(
    orderstr: &str,
    order: u32,
    sizes: Vec<u32>,
    samples: u32,
    included_funcs: impl Fn(SetType, u32) -> Vec<usize>,
) -> Result<(), std::io::Error> {
    const LANGUAGE: &str = "rustn";

    let published = sizes.len() > 1;
    let name = format!("bron_kerbosch_{}_order_{}", LANGUAGE, orderstr);
    let temppath = Path::new("tmp").with_extension("csv");
    {
        let mut writer: Option<csv::Writer<File>> = if published {
            let file = File::create(&temppath)?;
            let mut wtr = csv::Writer::from_writer(file);
            wtr.write_record(
                ["Size"].iter().map(|&s| String::from(s)).chain(
                    SetType::iter()
                        .cartesian_product(FUNC_NAMES.iter())
                        .flat_map(|(set_type, name)| {
                            vec![
                                format!("{}@{} min", name, set_type),
                                format!("{}@{} mean", name, set_type),
                                format!("{}@{} max", name, set_type),
                            ]
                        }),
                ),
            )?;
            Some(wtr)
        } else {
            None
        };
        for size in sizes {
            let mut stats: BTreeMap<SetType, [SampleStatistics<Seconds>; NUM_FUNCS]> =
                BTreeMap::new();
            for set_type in SetType::iter() {
                stats.insert(
                    set_type,
                    bk_core(order, size, samples, &included_funcs, set_type),
                );
            }
            if let Some(mut wtr) = writer.as_mut() {
                wtr.write_record([size].iter().map(|&i| i.to_string()).chain(
                    SetType::iter().cartesian_product(0..NUM_FUNCS).flat_map(
                        |(set_type, func_index)| {
                            let s = &stats[&set_type][func_index];
                            vec![
                                s.min().to_string(),
                                s.mean().to_string(),
                                s.max().to_string(),
                            ]
                        },
                    ),
                ))?;
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
        let sizes_10k = (100_000..=800_000).step_by(100_000); // max 49_995_000
        let sizes_1m = std::iter::empty()
            .chain((10_000..50_000).step_by(10_000))
            .chain((50_000..200_000).step_by(50_000))
            .chain((200_000..1_000_000).step_by(200_000))
            .chain((1_000_000..=5_000_000).step_by(1_000_000));
        let all_funcs = |_set_type: SetType, _size: u32| -> Vec<usize> { (0..NUM_FUNCS).collect() };
        bk("100", 100, sizes_100.collect(), 5, all_funcs)?;
        thread::sleep(Duration::from_secs(7));
        bk(
            "10k",
            10_000,
            sizes_10k.collect(),
            3,
            |set_type: SetType, size: u32| -> Vec<usize> {
                (0..NUM_FUNCS)
                    .filter(|func_index| match func_index {
                        0 | 4 | 8 => size <= 500_000 || set_type == SetType::Fnv,
                        _ => true,
                    })
                    .collect()
            },
        )?;
        thread::sleep(Duration::from_secs(7));
        bk(
            "1M",
            1_000_000,
            sizes_1m.collect(),
            3,
            |set_type: SetType, size: u32| -> Vec<usize> {
                (0..NUM_FUNCS)
                    .filter(|func_index| {
                        match func_index {
                            9 => true,
                            0 => false, // No need to keep testing the unimproved one
                            1 => {
                                set_type == SetType::BTreeSet
                                    || size <= 40_000
                                    || (set_type == SetType::Hashbrown && size <= 150_000)
                            }
                            _ => set_type == SetType::BTreeSet && size <= 2_000_000,
                        }
                    })
                    .collect()
            },
        )?;
    } else if !opt.order.is_empty() && !opt.sizes.is_empty() {
        let order = parse_positive_int(&opt.order);
        let sizes = opt.sizes.iter().map(|s| parse_positive_int(&s));
        let included_funcs = |set_type: SetType, _size: u32| -> Vec<usize> {
            if opt.set.filter(|&s| s != set_type).is_some() {
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
