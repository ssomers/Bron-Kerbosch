use crate::utils;
use anyhow::{Context, Result, anyhow};
use bron_kerbosch::{Adjacencies, Graph, Vertex, VertexSetLike};
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Not;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
pub enum Size {
    Of(usize),
}

fn locator(path: &Path, line_num: usize) -> String {
    let mut locator = format!("In file {}", path.display());
    if line_num > 0 {
        write!(locator, " on line {line_num}").ok();
    }
    locator
}

fn read_edges<VertexSet>(path: &Path, orderstr: &str, size: usize) -> Result<Adjacencies<VertexSet>>
where
    VertexSet: VertexSetLike + Clone,
{
    let order = utils::parse_positive_int(orderstr);
    let mut adjacencies = Adjacencies::new(VertexSet::new(), order);
    let context = |line_num| {
        move || {
            format!(
                "{}\nPerhaps (re)generate with `python -m random_graph {orderstr} <max_size?>`",
                locator(path, line_num)
            )
        }
    };
    let f = File::open(path).with_context(context(0))?;
    let reader = BufReader::new(f);
    let mut line_num = 0;
    for line_result in reader.lines().take(size) {
        line_num += 1;
        let line = line_result.with_context(context(line_num))?;
        let mut split = line.split(' ');
        let v = split.next().expect("at least an empty string");
        let w = split
            .next()
            .ok_or_else(|| anyhow!("Missing 2nd field"))
            .with_context(context(line_num))?;
        let v = Vertex::new(v.parse().with_context(context(line_num))?);
        let w = Vertex::new(w.parse().with_context(context(line_num))?);
        debug_assert_ne!(v, w);
        debug_assert!(adjacencies[v].contains(w).not());
        debug_assert!(adjacencies[w].contains(v).not());
        adjacencies[v].insert(w);
        adjacencies[w].insert(v);
    }
    if line_num < size {
        return Err(anyhow!("Exhausted generated list of edges")).with_context(context(line_num));
    }
    Ok(adjacencies)
}

pub struct KnownCliqueCounts {
    pub size_at_least_2: usize,
    pub size_at_least_3: usize,
}

fn read_clique_counts(
    path: &Path,
    orderstr: &str,
    size: usize,
) -> Result<Option<KnownCliqueCounts>> {
    let f = File::open(path).with_context(|| locator(path, 0))?;
    let reader = BufReader::new(f);
    let context = |line_num| move || locator(path, line_num);
    let prefix = format!("{orderstr}\t{size}\t");
    for (line_idx, line_result) in reader.lines().enumerate().skip(1) {
        let line = line_result.with_context(context(line_idx + 1))?;
        if line.starts_with(&prefix) {
            let mut c: Vec<_> = line[prefix.len()..]
                .split('\t')
                .map(|s| Some(s.parse::<usize>().with_context(context(line_idx + 1))))
                .collect();
            return Ok(Some(KnownCliqueCounts {
                size_at_least_2: c[0].take().unwrap()?,
                size_at_least_3: c[1].take().unwrap()?,
            }));
        }
    }
    Ok(None)
}

pub fn read_undirected<VertexSet>(
    orderstr: &str,
    size: Size,
) -> Result<(Graph<VertexSet>, Option<KnownCliqueCounts>)>
where
    VertexSet: VertexSetLike + Clone,
{
    let order = utils::parse_positive_int(orderstr);
    assert!(order > 0);
    let Size::Of(size) = size;
    let fully_meshed_size = order * (order - 1) / 2;
    if size > fully_meshed_size {
        return Err(anyhow!(
            "{} nodes accommodate at most {} edges",
            order,
            fully_meshed_size
        ));
    }

    let edges_name = &format!("random_edges_order_{orderstr}.txt");
    let stats_name = "random_stats.txt";
    let edges_pbuf: PathBuf = ["..", "data", edges_name].iter().collect();
    let stats_pbuf: PathBuf = ["..", "data", stats_name].iter().collect();
    let adjacencies = read_edges(edges_pbuf.as_path(), orderstr, size)?;
    let clique_counts = read_clique_counts(stats_pbuf.as_path(), orderstr, size)?;

    let graph = Graph::new(adjacencies);
    assert_eq!(graph.order(), order);
    assert_eq!(graph.size(), size);
    Ok((graph, clique_counts))
}
