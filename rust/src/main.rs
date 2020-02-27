#![allow(
    clippy::assertions_on_constants,
    clippy::or_fun_call,
    clippy::manual_mul_add
)]

mod random_graph;

use bron_kerbosch::graph::{NewableUndirectedGraph, Vertex, VertexSetLike};
use bron_kerbosch::reporter::SimpleReporter;
use bron_kerbosch::slimgraph::SlimUndirectedGraph;
use bron_kerbosch::{explore, order_cliques, OrderedCliques, FUNC_NAMES, NUM_FUNCS};
use random_graph::{parse_positive_int, read_undirected, Size};
use stats::SampleStatistics;

use fnv::FnvHashSet;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};
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
    HashSet,
    #[strum(to_string = "hashbrown")]
    Hashbrown,
    #[strum(to_string = "fnv")]
    Fnv,
    BTreeSet,
    #[strum(to_string = "ord_vec")]
    OrdVec,
}

type Seconds = f32;

fn read_random_graph<VertexSet, G>(set_type: SetType, orderstr: &str, size: Size) -> G
where
    VertexSet: VertexSetLike + Clone,
    G: NewableUndirectedGraph<VertexSet>,
{
    let Size::Of(size) = size;
    let instant = Instant::now();
    let graph: G = read_undirected(orderstr, Size::Of(size)).unwrap();
    let seconds = instant.elapsed().as_secs_f32();
    println!(
        "{}-based random graph of order {}, size {}: (generating took {:.3}s)",
        set_type, orderstr, size, seconds
    );
    graph
}

fn bron_kerbosch_timed<VertexSet>(
    graph: &SlimUndirectedGraph<VertexSet>,
    samples: u32,
    func_indices: &[usize],
) -> [SampleStatistics<Seconds>; NUM_FUNCS]
where
    VertexSet: VertexSetLike + Send + Sync,
{
    let mut times: [SampleStatistics<Seconds>; NUM_FUNCS] = Default::default();
    let mut first: Option<OrderedCliques> = None;
    for sample in 0..samples {
        for &func_index in func_indices {
            let mut reporter = SimpleReporter::new();
            let instant = Instant::now();
            explore(func_index, graph, &mut reporter);
            let secs: Seconds = instant.elapsed().as_secs_f32();
            if samples > 1 && secs >= 3.0 {
                println!("  {:8}: {}s", FUNC_NAMES[func_index], secs);
            }
            if sample < 2 {
                let current = order_cliques(reporter.cliques);
                for first_result in first.iter() {
                    if *first_result != current {
                        eprintln!(
                            "  {:8}: expected {} cliques, obtained {} different cliques",
                            FUNC_NAMES[func_index],
                            first_result.len(),
                            current.len()
                        );
                    }
                }
                first = first.or(Some(current))
            }

            times[func_index].put(secs);
        }
    }
    times
}

fn bk_core_core<VertexSet>(
    orderstr: &str,
    size: u32,
    samples: u32,
    set_type: SetType,
    func_indices: &[usize],
) -> [SampleStatistics<Seconds>; NUM_FUNCS]
where
    VertexSet: VertexSetLike + Clone + Sync + Send,
{
    let graph: SlimUndirectedGraph<VertexSet> =
        read_random_graph(set_type, orderstr, Size::Of(size));
    bron_kerbosch_timed(&graph, samples, func_indices)
}

fn bk_core(
    orderstr: &str,
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
                bk_core_core::<BTreeSet<Vertex>>(orderstr, size, samples, set_type, &func_indices)
            }
            SetType::HashSet => {
                bk_core_core::<HashSet<Vertex>>(orderstr, size, samples, set_type, &func_indices)
            }
            SetType::Fnv => {
                bk_core_core::<FnvHashSet<Vertex>>(orderstr, size, samples, set_type, &func_indices)
            }
            SetType::Hashbrown => bk_core_core::<hashbrown::HashSet<Vertex>>(
                orderstr,
                size,
                samples,
                set_type,
                &func_indices,
            ),
            SetType::OrdVec => {
                bk_core_core::<Vec<Vertex>>(orderstr, size, samples, set_type, &func_indices)
            }
        };
        for func_index in func_indices {
            let name = FUNC_NAMES[func_index];
            let mean = stats[func_index].mean();
            if !mean.is_nan() {
                let reldev = stats[func_index].deviation() / mean;
                println!("{:8}: {:6.3}s ± {:.0}%", name, mean, 100.0 * reldev);
            }
        }
        stats
    }
}

fn bk(
    orderstr: &str,
    sizes: impl Iterator<Item = u32>,
    samples: u32,
    included_funcs: impl Fn(SetType, u32) -> Vec<usize>,
) -> Result<(), std::io::Error> {
    const LANGUAGE: &str = "rust";

    let sizes: Vec<_> = sizes.collect();
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
                    bk_core(orderstr, size, samples, &included_funcs, set_type),
                );
            }
            if let Some(wtr) = writer.as_mut() {
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
        bk(
            "100",
            (2_000..=3_000).step_by(50), // max 4_950
            5,
            |_set_type: SetType, _size: u32| -> Vec<usize> { (0..NUM_FUNCS).collect() },
        )?;
        thread::sleep(Duration::from_secs(7));
        bk(
            "10k",
            std::iter::empty()
                .chain((10_000..100_000).step_by(10_000))
                .chain((100_000..=200_000).step_by(25_000)),
            3,
            |_set_type: SetType, _size: u32| -> Vec<usize> {
                // Skip Ver1 (already rejected) and Ver2+RP (not interesting in random graph)
                vec![2, 3, 4, 6, 7, 8, 9]
            },
        )?;
        thread::sleep(Duration::from_secs(7));
        bk(
            "1M",
            std::iter::empty()
                .chain((10_000..100_000).step_by(10_000))
                .chain((100_000..250_000).step_by(50_000))
                .chain((250_000..2_000_000).step_by(250_000))
                .chain((2_000_000..=5_000_000).step_by(1_000_000)),
            3,
            |set_type: SetType, size: u32| -> Vec<usize> {
                match size {
                    0..=99_999 => vec![1],
                    100_000..=249_999 => match set_type {
                        SetType::OrdVec => vec![],
                        SetType::BTreeSet => vec![1, 7, 9],
                        _ => vec![7, 9],
                    },
                    250_000..=1_500_000 => match set_type {
                        SetType::OrdVec => vec![],
                        _ => vec![7, 9],
                    },
                    _ => match set_type {
                        SetType::OrdVec => vec![],
                        _ => vec![9],
                    },
                }
            },
        )?;
    } else if !opt.order.is_empty() && !opt.sizes.is_empty() {
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
        bk(&opt.order, sizes, 1, included_funcs)?;
    } else {
        eprintln!("Specify order and size(s)")
    }
    Ok(())
}
