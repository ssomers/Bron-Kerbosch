//! Naive Bron-Kerbosch algorithm, optimized

use graph::{UndirectedGraph, Vertex};
use reporter::Reporter;

use std::collections::HashSet;

pub fn explore(graph: &UndirectedGraph, reporter: &mut Reporter) {
    let candidates = graph.connected_nodes();
    if !candidates.is_empty() {
        visit(graph, reporter, candidates, HashSet::new(), Clique::Empty);
    }
}

enum Clique<'a> {
    Empty,
    Cons(&'a Clique<'a>, Vertex),
}

impl<'a> Clique<'a> {
    fn collect(&self) -> Vec<Vertex> {
        let mut clique: Vec<Vertex> = Vec::with_capacity(self.len());
        self.append_to(&mut clique);
        clique
    }
    fn len(&self) -> usize {
        match self {
            Clique::Empty => 0,
            Clique::Cons(c, _v) => c.len() + 1,
        }
    }
    fn append_to(&self, clique: &mut Vec<Vertex>) {
        match self {
            Clique::Empty => {}
            Clique::Cons(c, v) => {
                c.append_to(clique);
                clique.push(*v);
            }
        }
    }
}

fn visit(
    graph: &UndirectedGraph,
    reporter: &mut Reporter,
    mut candidates: HashSet<Vertex>,
    mut excluded: HashSet<Vertex>,
    clique: Clique,
) {
    debug_assert!(candidates.iter().all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.iter().all(|&v| graph.degree(v) > 0));
    reporter.inc_count();
    if candidates.is_empty() && excluded.is_empty() {
        reporter.record(clique.collect());
        return;
    }

    while let Some(v) = remove_arbitrary(&mut candidates) {
        let neighbours = graph.adjacencies(v);
        debug_assert!(!neighbours.is_empty());
        let neighbouring_candidates = neighbours.intersection(&candidates).cloned().collect();
        let neighbouring_excluded = neighbours.intersection(&excluded).cloned().collect();
        visit(
            graph,
            reporter,
            neighbouring_candidates,
            neighbouring_excluded,
            Clique::Cons(&clique, v),
        );
        excluded.insert(v);
    }
}

fn remove_arbitrary(s: &mut HashSet<Vertex>) -> Option<Vertex> {
    s.iter().next().cloned().map(|v| {
        s.remove(&v);
        v
    })
}
