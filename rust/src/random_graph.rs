use anyhow::{anyhow, Context, Result};
use bron_kerbosch::{Adjacencies, NewableUndirectedGraph, Vertex, VertexSetLike};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Copy)]
pub enum Size {
    Of(usize),
}

pub fn parse_positive_int(value: &str) -> usize {
    let (numstr, factor) = if let Some(megas) = value.strip_suffix('M') {
        (megas, 1_000_000)
    } else if let Some(kilos) = value.strip_suffix('k') {
        (kilos, 1_000)
    } else {
        (value, 1)
    };
    let num: usize = numstr
        .parse()
        .unwrap_or_else(|err| panic!("{} is not a positive integer ({})", numstr, err));
    num * factor
}

fn read_edges<VertexSet>(path: &Path, orderstr: &str, size: usize) -> Result<Adjacencies<VertexSet>>
where
    VertexSet: VertexSetLike + Clone,
{
    let order = parse_positive_int(orderstr);
    let mut adjacencies = Adjacencies::new(VertexSet::new(), order);
    let context = |line_num| {
        move || {
            let line_str = if line_num > 0 {
                format!(" on line {}", line_num)
            } else {
                String::new()
            };
            format!(
                "In file {}{}\nPerhaps (re)generate with `python -m random_graph {} <max_size?>`",
                path.display(),
                line_str,
                orderstr
            )
        }
    };
    let f = File::open(path).with_context(context(0))?;
    let reader = BufReader::new(f);
    let mut line_num = 0;
    for line_result in reader.lines().take(size as usize) {
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
        debug_assert!(!adjacencies[v].contains(w));
        debug_assert!(!adjacencies[w].contains(v));
        adjacencies[v].insert(w);
        adjacencies[w].insert(v);
    }
    if line_num < size {
        return Err(anyhow!("Exhausted generated list of edges")).with_context(context(line_num));
    }
    Ok(adjacencies)
}

fn read_clique_count(path: &Path, orderstr: &str, size: usize) -> Result<Option<usize>> {
    let f = File::open(path).with_context(|| format!("In file {}", path.display()))?;
    let reader = BufReader::new(f);
    let context = |line_num| move || format!("In file {} on line {}", path.display(), line_num);
    let prefix = format!("{}\t{}\t", orderstr, size);
    for (line_idx, line_result) in reader.lines().enumerate().skip(1) {
        let line = line_result.with_context(context(line_idx + 1))?;
        if line.starts_with(&prefix) {
            let c: usize = line[prefix.len()..]
                .parse()
                .with_context(context(line_idx + 1))?;
            return Ok(Some(c));
        }
    }
    Ok(None)
}

pub fn read_undirected<VertexSet, G>(orderstr: &str, size: Size) -> Result<(G, Option<usize>)>
where
    VertexSet: VertexSetLike + Clone,
    G: NewableUndirectedGraph<VertexSet>,
{
    let order = parse_positive_int(orderstr);
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

    let edges_name = &format!("random_edges_order_{}.txt", orderstr);
    let stats_name = "random_stats.txt";
    let edges_pbuf: PathBuf = ["..", "data", edges_name].iter().collect();
    let stats_pbuf: PathBuf = ["..", "data", stats_name].iter().collect();
    let adjacencies = read_edges(edges_pbuf.as_path(), orderstr, size)?;
    let clique_count = read_clique_count(stats_pbuf.as_path(), orderstr, size)?;

    let g = G::new(adjacencies);
    assert_eq!(g.order(), order);
    assert_eq!(g.size(), size);
    Ok((g, clique_count))
}
