use anyhow::{anyhow, Context, Result};
use bron_kerbosch::graph::{Adjacencies, NewableUndirectedGraph, Vertex, VertexSetLike};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub enum Size {
    Of(u32),
}

pub fn parse_positive_int(value: &str) -> u32 {
    let numstr: &str;
    let factor: u32;
    if value.ends_with('M') {
        numstr = &value[0..value.len() - 1];
        factor = 1_000_000;
    } else if value.ends_with('k') {
        numstr = &value[0..value.len() - 1];
        factor = 1_000;
    } else {
        numstr = value;
        factor = 1;
    }
    let num: u32 = numstr
        .parse()
        .unwrap_or_else(|err| panic!("{} is not a positive integer ({})", numstr, err));
    num * factor
}

fn new_adjacencies<VertexSet>(order: u32) -> Adjacencies<VertexSet>
where
    VertexSet: VertexSetLike + Clone,
{
    std::vec::from_elem(VertexSet::new(), order as usize)
}

fn read_edges<VertexSet>(path: &Path, orderstr: &str, size: u32) -> Result<Vec<VertexSet>>
where
    VertexSet: VertexSetLike + Clone,
{
    let order = parse_positive_int(orderstr);
    let mut adjacency_sets: Vec<VertexSet> = new_adjacencies(order);
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
        let v: Vertex = v.parse().with_context(context(line_num))?;
        let w: Vertex = w.parse().with_context(context(line_num))?;
        debug_assert_ne!(v, w);
        debug_assert!(!adjacency_sets[v as usize].contains(w));
        debug_assert!(!adjacency_sets[w as usize].contains(v));
        adjacency_sets[v as usize].insert(w);
        adjacency_sets[w as usize].insert(v);
    }
    if line_num < size {
        return Err(anyhow!("Exhausted generated list of edges")).with_context(context(line_num));
    }
    Ok(adjacency_sets)
}

fn read_clique_count(path: &Path, orderstr: &str, size: u32) -> Result<Option<usize>> {
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

    let dir = Path::new("..");
    let edges_name = &format!("random_edges_order_{}", orderstr);
    let stats_name = "random_stats";
    let edges_path = Path::join(dir, Path::new(edges_name).with_extension("txt"));
    let stats_path = Path::join(dir, Path::new(stats_name).with_extension("txt"));
    let adjacency_sets = read_edges(&edges_path, orderstr, size)?;
    let clique_count = read_clique_count(&stats_path, orderstr, size)?;

    let g = G::new(adjacency_sets);
    assert_eq!(g.order(), order);
    assert_eq!(g.size(), size);
    Ok((g, clique_count))
}
