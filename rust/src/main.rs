mod known_random_graph;
mod utils;

use bron_kerbosch::clique_consumers::{CliqueCollector, CliqueCounter};
use bron_kerbosch::{FUNC_NAMES, OrderedCliques, Vertex, VertexSetLike, explore, order_cliques};
use known_random_graph::{Size, read_undirected};
use stats::SampleStatistics;

use clap::{arg, command};
use fnv::FnvHashSet;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::iter::once;
use std::ops::Not;
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

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
enum Run {
    OneOff,
    WarmUp,
    Regular,
}

const NUM_FUNCS: usize = FUNC_NAMES.len();
const CLIQUE_MIN_SIZE: usize = 3;
const NUM_VISITING_THREADS: usize = 5;

type Seconds = f32;

fn bron_kerbosch_timed<VertexSet: VertexSetLike + Clone + Sync>(
    run: Run,
    set_type: SetType,
    orderstr: &str,
    size: usize,
    func_indices: &[usize],
    timed_samples: u32,
) -> [SampleStatistics<Seconds>; NUM_FUNCS] {
    let instant = Instant::now();
    let (graph, known_clique_count) =
        read_undirected::<VertexSet>(orderstr, Size::Of(size)).unwrap();
    let known_clique_count = if run == Run::OneOff {
        None
    } else {
        let counts = known_clique_count.expect("no known clique count");
        match CLIQUE_MIN_SIZE {
            2 => Some(counts.size_at_least_2),
            3 => Some(counts.size_at_least_3),
            _ => unreachable!(),
        }
    };
    let seconds = instant.elapsed().as_secs_f32();
    if run == Run::Regular {
        println!(
            "{}-based random graph of order {}, {} edges, {} cliques: (generating took {:.3}s)",
            set_type,
            orderstr,
            utils::formatted(size),
            utils::formatted(known_clique_count.unwrap()),
            seconds
        );
    }

    let mut times: [SampleStatistics<Seconds>; NUM_FUNCS] = Default::default();
    let mut first: Option<OrderedCliques> = None;

    for sample in 0..=timed_samples {
        for &func_index in func_indices {
            if sample == 0 {
                let consumer = CliqueCollector::new(CLIQUE_MIN_SIZE);
                let cliques = utils::do_timely(
                    || explore(func_index, &graph, consumer, NUM_VISITING_THREADS),
                    format!("{} is still busy collecting", FUNC_NAMES[func_index]),
                );
                let current = order_cliques(cliques.into_iter());
                if run == Run::OneOff {
                    println!(
                        "{} cliques found by {}",
                        current.len(),
                        FUNC_NAMES[func_index]
                    );
                }
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
                    if let Some(known_clique_count) = known_clique_count
                        && current.len() != known_clique_count
                    {
                        eprintln!(
                            "  {:8}: expected {} cliques, obtained {} cliques",
                            FUNC_NAMES[func_index],
                            known_clique_count,
                            current.len()
                        );
                    }
                    first = Some(current);
                }
            } else {
                let consumer = CliqueCounter::new(CLIQUE_MIN_SIZE);
                let instant = Instant::now();
                let clique_count = explore(func_index, &graph, consumer, NUM_VISITING_THREADS);
                let secs: Seconds = instant.elapsed().as_secs_f32();
                if let Some(known_clique_count) = known_clique_count
                    && clique_count != known_clique_count
                {
                    eprintln!(
                        "  {:8}: expected {} cliques, obtained {} cliques",
                        FUNC_NAMES[func_index], known_clique_count, clique_count
                    );
                }
                times[func_index].put(secs);
            }
        }
    }
    if run == Run::Regular {
        for &func_index in func_indices {
            let func_name = FUNC_NAMES[func_index];
            let mean = times[func_index].mean();
            let reldev = times[func_index].deviation() / mean;
            println!("{:10} {:6.3}s ± {:.0}%", func_name, mean, 100.0 * reldev);
        }
    }
    times
}

fn bk_core(
    run: Run,
    set_type: SetType,
    orderstr: &str,
    size: usize,
    func_indices: &[usize],
    timed_samples: u32,
) -> [SampleStatistics<Seconds>; NUM_FUNCS] {
    match set_type {
        SetType::BTreeSet => bron_kerbosch_timed::<BTreeSet<Vertex>>(
            run,
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
        SetType::HashSet => bron_kerbosch_timed::<HashSet<Vertex>>(
            run,
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
        SetType::Fnv => bron_kerbosch_timed::<FnvHashSet<Vertex>>(
            run,
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
        SetType::Hashbrown => bron_kerbosch_timed::<hashbrown::HashSet<Vertex>>(
            run,
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
        SetType::OrdVec => bron_kerbosch_timed::<Vec<Vertex>>(
            run,
            set_type,
            orderstr,
            size,
            func_indices,
            timed_samples,
        ),
    }
}

fn bk(
    run: Run,
    orderstr: &str,
    sizes: impl Iterator<Item = usize>,
    timed_samples: u32,
    included_funcs: impl Fn(SetType, usize) -> Vec<usize>,
) -> Result<(), std::io::Error> {
    let sizes = Vec::from_iter(sizes);
    let temppath = Path::new("tmp").with_extension("csv");
    {
        let mut writer: Option<csv::Writer<File>> = if run == Run::OneOff {
            None
        } else {
            Some(csv::Writer::from_writer(File::create(&temppath)?))
        };
        let mut set_types_used = vec![];
        for size in sizes {
            let mut stats: BTreeMap<SetType, [SampleStatistics<Seconds>; NUM_FUNCS]> =
                BTreeMap::new();
            for set_type in SetType::iter() {
                let func_indices = included_funcs(set_type, size);
                if func_indices.is_empty().not() {
                    stats.insert(
                        set_type,
                        bk_core(run, set_type, orderstr, size, &func_indices, timed_samples),
                    );
                }
            }
            if let Some(wtr) = writer.as_mut() {
                if set_types_used.is_empty() {
                    set_types_used = stats.keys().copied().collect();
                    assert!(set_types_used.is_empty().not());
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
                                    .filter(|&s| s.is_empty().not())
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

    if run == Run::Regular {
        let name = Path::new(&format!("random_time_rust_order_{orderstr}")).with_extension("csv");
        let path = Path::new("..").join(Path::new("data")).join(name);
        std::fs::rename(temppath, path)?;
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let matches = command!()
        .arg(arg!(-v - -ver[VER]).value_parser(clap::value_parser!(usize)))
        .arg(arg!(-s - -set[SET]))
        .arg(arg!([order]))
        .arg(arg!([size] ... ))
        .get_matches();
    if matches.args_present().not() {
        bk(Run::WarmUp, "100", once(2_000), 3, |_, _| {
            (0..NUM_FUNCS).collect()
        })?;
        debug_assert!(false, "Run with --release for meaningful measurements");
        thread::sleep(Duration::from_secs(3));
        bk(
            Run::Regular,
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
        thread::sleep(Duration::from_secs(3));
        bk(
            Run::Regular,
            "10k",
            std::iter::empty()
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
        thread::sleep(Duration::from_secs(3));
        bk(
            Run::Regular,
            "1M",
            std::iter::empty()
                .chain((250_000..2_000_000).step_by(250_000))
                .chain((2_000_000..=5_000_000).step_by(1_000_000)),
            3,
            |_set_type: SetType, _size: usize| -> Vec<usize> { vec![4, 7, 9] },
        )?;
    } else if let (Some(order), Some(sizes)) = (
        matches.get_one::<String>("order"),
        matches.get_many::<String>("size"),
    ) {
        let forced_set_type = matches
            .get_one::<String>("set")
            .map(|t| SetType::from_str(t).unwrap());
        let sizes = sizes.map(|s| utils::parse_positive_int(s.as_str()));
        let included_funcs = |set_type: SetType, _size: usize| -> Vec<usize> {
            if forced_set_type.is_some() && forced_set_type != Some(set_type) {
                vec![]
            } else if let Some(&func_index) = matches.get_one::<usize>("ver") {
                vec![func_index]
            } else {
                (0..NUM_FUNCS).collect()
            }
        };
        bk(Run::OneOff, order, sizes, 0, included_funcs)?;
    } else {
        eprintln!("Specify order and size(s)")
    }
    Ok(())
}
