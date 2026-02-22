mod random_graph;

use bron_kerbosch::{CliqueCollector, CliqueCounter};
use bron_kerbosch::{FUNC_NAMES, OrderedCliques, explore, order_cliques};
use bron_kerbosch::{SlimUndirectedGraphFactory, Vertex, VertexSetLike};
use random_graph::{Size, parse_positive_int, read_undirected};
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

// Source - https://stackoverflow.com/a/67834588
fn formatted(num: usize) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("_")
}

fn bron_kerbosch_timed<VertexSet: VertexSetLike + Clone>(
    set_type: SetType,
    orderstr: &str,
    size: usize,
    func_indices: &[usize],
    timed_samples: u32,
) -> [SampleStatistics<Seconds>; NUM_FUNCS] {
    let instant = Instant::now();
    let known =
        read_undirected::<VertexSet, SlimUndirectedGraphFactory>(orderstr, Size::Of(size)).unwrap();
    let seconds = instant.elapsed().as_secs_f32();
    let known_clique_count_str = known.clique_count.map_or(String::from("?"), formatted);
    println!(
        "{}-based random graph of order {}, {} edges, {} cliques: (generating took {:.3}s)",
        set_type,
        orderstr,
        formatted(size),
        known_clique_count_str,
        seconds
    );

    let mut times: [SampleStatistics<Seconds>; NUM_FUNCS] = Default::default();
    let mut first: Option<OrderedCliques> = None;
    // If we're genuinely sampling, first warm up.
    for sample in 0..=timed_samples {
        for &func_index in func_indices {
            let mut collecting_reporter = CliqueCollector::default();
            let mut counting_reporter = CliqueCounter::default();
            let instant = Instant::now();
            if sample == 0 {
                explore(func_index, &known.graph, &mut collecting_reporter);
            } else {
                explore(func_index, &known.graph, &mut counting_reporter);
            }
            let secs: Seconds = instant.elapsed().as_secs_f32();
            if sample == 0 {
                if timed_samples == 0 || secs >= 3.0 {
                    println!("  {:10} {}s", FUNC_NAMES[func_index], secs);
                }
                let current = order_cliques(collecting_reporter.cliques.into_iter());
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
                    if let Some(clique_count) = known.clique_count {
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
            } else if let Some(clique_count) = known.clique_count {
                if counting_reporter.count != clique_count {
                    eprintln!(
                        "  {:8}: expected {} cliques, obtained {} cliques",
                        FUNC_NAMES[func_index], clique_count, counting_reporter.count
                    );
                }
                times[func_index].put(secs);
            }
        }
    }
    for &func_index in func_indices {
        let func_name = FUNC_NAMES[func_index];
        let mean = times[func_index].mean();
        let reldev = times[func_index].deviation() / mean;
        println!("{:10} {:6.3}s ± {:.0}%", func_name, mean, 100.0 * reldev);
    }
    times
}

fn bk_core(
    set_type: SetType,
    orderstr: &str,
    size: usize,
    func_indices: &[usize],
    timed_samples: u32,
) -> [SampleStatistics<Seconds>; NUM_FUNCS] {
    match set_type {
        SetType::BTreeSet => bron_kerbosch_timed::<BTreeSet<Vertex>>(
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
        SetType::HashSet => bron_kerbosch_timed::<HashSet<Vertex>>(
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
        SetType::Fnv => bron_kerbosch_timed::<FnvHashSet<Vertex>>(
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
        SetType::Hashbrown => bron_kerbosch_timed::<hashbrown::HashSet<Vertex>>(
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
        SetType::OrdVec => bron_kerbosch_timed::<Vec<Vertex>>(
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
    }
}

fn bk(
    orderstr: &str,
    sizes: impl Iterator<Item = usize>,
    timed_samples: u32,
    included_funcs: impl Fn(SetType, usize) -> Vec<usize>,
) -> Result<(), std::io::Error> {
    const LANGUAGE: &str = "rust";

    let sizes = Vec::from_iter(sizes);
    let published = sizes.len() > 1;
    let name = format!("bron_kerbosch_{LANGUAGE}_order_{orderstr}");
    let temppath = Path::new("tmp").with_extension("csv");
    {
        let mut writer: Option<csv::Writer<File>> = if published {
            Some(csv::Writer::from_writer(File::create(&temppath)?))
        } else {
            None
        };
        let mut set_types_used = vec![];
        for size in sizes {
            let mut stats: BTreeMap<SetType, [SampleStatistics<Seconds>; NUM_FUNCS]> =
                BTreeMap::new();
            for set_type in SetType::iter() {
                let func_indices = included_funcs(set_type, size);
                if !func_indices.is_empty() {
                    stats.insert(
                        set_type,
                        bk_core(set_type, orderstr, size, &func_indices, timed_samples),
                    );
                }
            }
            if let Some(wtr) = writer.as_mut() {
                if set_types_used.is_empty() {
                    set_types_used = stats.keys().copied().collect();
                    assert!(!set_types_used.is_empty());
                    wtr.write_record(
                        ["Size"].iter().map(|&s| String::from(s)).chain(
                            set_types_used
                                .iter()
                                .cartesian_product(FUNC_NAMES.iter())
                                .flat_map(|(set_type, name)| {
                                    vec![
                                        format!("{name}@{set_type} min"),
                                        format!("{name}@{set_type} mean"),
                                        format!("{name}@{set_type} max"),
                                    ]
                                }),
                        ),
                    )?;
                }
                wtr.write_record(
                    [size].iter().map(|&i| i.to_string()).chain(
                        set_types_used
                            .iter()
                            .cartesian_product(0..NUM_FUNCS)
                            .flat_map(|(set_type, func_index)| {
                                if let Some(s) = &stats
                                    .get(set_type)
                                    .as_ref()
                                    .map(|&s| &s[func_index])
                                    .filter(|&s| !s.is_empty())
                                {
                                    vec![
                                        s.min().to_string(),
                                        s.mean().to_string(),
                                        s.max().to_string(),
                                    ]
                                } else {
                                    vec![String::new(); 3]
                                }
                            }),
                    ),
                )?;
            }
        }
    }

    if published {
        let path = Path::join(Path::new(".."), Path::new(&name).with_extension("csv"));
        std::fs::rename(temppath, path)?;
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
            //.chain((5_000..=500_000).step_by(495_000)),
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
    } else if let (Some(order), Some(sizes)) = (
        matches.get_one::<String>("order"),
        matches.get_many::<String>("size"),
    ) {
        let forced_set_type = matches
            .get_one::<String>("set")
            .map(|t| SetType::from_str(t).unwrap());
        let sizes = sizes.map(|s| parse_positive_int(s.as_str()));
        let included_funcs = |set_type: SetType, _size: usize| -> Vec<usize> {
            if forced_set_type.is_some() && forced_set_type != Some(set_type) {
                vec![]
            } else if let Some(&func_index) = matches.get_one::<usize>("ver") {
                vec![func_index]
            } else {
                (0..NUM_FUNCS).collect()
            }
        };
        bk(order, sizes, 0, included_funcs)?;
    } else {
        eprintln!("Specify order and size(s)")
    }
    Ok(())
}
