mod random_graph;

use bron_kerbosch::{explore, order_cliques, OrderedCliques, FUNC_NAMES};
use bron_kerbosch::{CountingReporter, SimpleReporter};
use bron_kerbosch::{NewableUndirectedGraph, SlimUndirectedGraph, Vertex, VertexSetLike};
use random_graph::{parse_positive_int, read_undirected, Size};
use stats::SampleStatistics;

use clap::{arg, command};
use fnv::FnvHashSet;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Copy, Clone, Debug, Display, EnumIter, EnumString, Eq, PartialEq, Ord, PartialOrd)]
enum SetType {
    #[strum(to_string = "Hash")]
    HashSet,
    #[strum(to_string = "hashbrown")]
    Hashbrown,
    #[strum(to_string = "fnv")]
    Fnv,
    #[strum(to_string = "BTree")]
    BTreeSet,
    #[strum(to_string = "ord_vec")]
    OrdVec,
}

const NUM_FUNCS: usize = FUNC_NAMES.len();
type Seconds = f32;

fn read_random_graph<VertexSet, G>(
    set_type: SetType,
    orderstr: &str,
    size: Size,
) -> (G, Option<usize>)
where
    VertexSet: VertexSetLike + Clone,
    G: NewableUndirectedGraph<VertexSet>,
{
    let instant = Instant::now();
    let (g, known_clique_count) = read_undirected(orderstr, size).unwrap();
    let seconds = instant.elapsed().as_secs_f32();
    let Size::Of(size) = size;
    println!(
        "{}-based random graph of order {}, size {}, {} cliques: (generating took {:.3}s)",
        set_type,
        orderstr,
        size,
        known_clique_count.map_or("?".to_string(), |c| c.to_string()),
        seconds
    );
    (g, known_clique_count)
}

fn bron_kerbosch_timed<VertexSet: VertexSetLike>(
    graph: &SlimUndirectedGraph<VertexSet>,
    known_clique_count: Option<usize>,
    samples: u32,
    func_indices: &[usize],
) -> [SampleStatistics<Seconds>; NUM_FUNCS] {
    let mut times: [SampleStatistics<Seconds>; NUM_FUNCS] = Default::default();
    let mut first: Option<OrderedCliques> = None;
    for sample in 0..=(if samples == 1 { 0 } else { samples }) {
        for &func_index in func_indices {
            if sample == 0 {
                let mut reporter = SimpleReporter::default();
                let instant = Instant::now();
                explore(func_index, graph, &mut reporter);
                let secs: Seconds = instant.elapsed().as_secs_f32();
                if secs >= 3.0 {
                    println!("  {:8}: {}s", FUNC_NAMES[func_index], secs);
                }
                let current = order_cliques(reporter.cliques.into_iter());
                if let Some(first_result) = first.as_ref() {
                    if *first_result != current {
                        eprintln!(
                            "  {:8}: expected {} cliques, obtained {} different cliques",
                            FUNC_NAMES[func_index],
                            first_result.len(),
                            current.len()
                        );
                    }
                } else {
                    if let Some(clique_count) = known_clique_count {
                        if current.len() != clique_count {
                            eprintln!(
                                "  {:8}: expected {} cliques, obtained {} cliques",
                                FUNC_NAMES[func_index],
                                clique_count,
                                current.len()
                            );
                        }
                    } else {
                        println!("  {} cliques", current.len());
                        return times;
                    }
                    first = Some(current);
                }
            } else if let Some(clique_count) = known_clique_count {
                let mut reporter = CountingReporter::default();
                let instant = Instant::now();
                explore(func_index, graph, &mut reporter);
                let secs: Seconds = instant.elapsed().as_secs_f32();
                if reporter.count != clique_count {
                    eprintln!(
                        "  {:8}: expected {} cliques, obtained {} cliques",
                        FUNC_NAMES[func_index], clique_count, reporter.count
                    );
                }
                times[func_index].put(secs);
            }
        }
    }
    times
}

fn bk_core_core<VertexSet: VertexSetLike + Clone>(
    orderstr: &str,
    size: usize,
    samples: u32,
    set_type: SetType,
    func_indices: &[usize],
) -> [SampleStatistics<Seconds>; NUM_FUNCS] {
    let (graph, known_clique_count): (SlimUndirectedGraph<VertexSet>, _) =
        read_random_graph(set_type, orderstr, Size::Of(size));
    bron_kerbosch_timed(&graph, known_clique_count, samples, func_indices)
}

fn bk_core(
    orderstr: &str,
    size: usize,
    samples: u32,
    included_funcs: &impl Fn(SetType, usize) -> Vec<usize>,
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
    sizes: impl Iterator<Item = usize>,
    samples: u32,
    included_funcs: impl Fn(SetType, usize) -> Vec<usize>,
) -> Result<(), std::io::Error> {
    const LANGUAGE: &str = "rust";

    let sizes = Vec::from_iter(sizes);
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
    let matches = command!()
        .arg(arg!(-v --ver <VER>).required(false))
        .arg(arg!(-s --set <SET>).required(false))
        .arg(arg!([order]))
        .arg(arg!([size] ... ))
        .get_matches();
    if !matches.args_present() {
        debug_assert!(false, "Run with --release for meaningful measurements");
        bk(
            "100",
            (2_000..=3_000).step_by(50), // max 4_950
            5,
            |set_type: SetType, _size: usize| -> Vec<usize> {
                match set_type {
                    SetType::HashSet => (0..NUM_FUNCS).collect(),
                    _ => vec![1, 4, 7, 9],
                }
            },
        )?;
        thread::sleep(Duration::from_secs(7));
        bk(
            "10k",
            std::iter::empty()
                .chain((1_000..10_000).step_by(1_000))
                .chain((10_000..100_000).step_by(10_000))
                .chain((100_000..=200_000).step_by(25_000)),
            3,
            |set_type: SetType, _size: usize| -> Vec<usize> {
                match set_type {
                    SetType::HashSet => (2..=9).collect(),
                    _ => vec![2, 4, 7, 9],
                }
            },
        )?;
        thread::sleep(Duration::from_secs(7));
        bk(
            "1M",
            std::iter::empty()
                .chain((10_000..50_000).step_by(10_000))
                .chain((50_000..250_000).step_by(50_000))
                .chain((250_000..2_000_000).step_by(250_000))
                .chain((2_000_000..=5_000_000).step_by(1_000_000)),
            3,
            |set_type: SetType, size: usize| -> Vec<usize> {
                match set_type {
                    SetType::BTreeSet if size > 3_000_000 => vec![],
                    SetType::OrdVec if size > 250_000 => vec![],
                    SetType::OrdVec if size > 100_000 => vec![4],
                    _ => vec![4, 7, 9],
                }
            },
        )?;
    } else if let (Some(order), Some(sizes)) =
        (matches.value_of("order"), matches.values_of("size"))
    {
        let forced_set_type = matches
            .value_of("set")
            .map(|t| SetType::from_str(t).unwrap());
        let sizes = sizes.map(parse_positive_int);
        let included_funcs = |set_type: SetType, _size: usize| -> Vec<usize> {
            if forced_set_type.is_some() && forced_set_type != Some(set_type) {
                vec![]
            } else if let Some(v) = matches.value_of("ver") {
                let func_index = usize::from_str(v).unwrap();
                vec![func_index]
            } else {
                (0..NUM_FUNCS).collect()
            }
        };
        bk(order, sizes, 1, included_funcs)?;
    } else {
        eprintln!("Specify order and size(s)")
    }
    Ok(())
}
