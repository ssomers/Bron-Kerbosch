//! Naive Bron-Kerbosch algorithm, optimized

use super::base::CliqueConsumer;
use super::graph::{UndirectedGraph, Vertex, VertexSetLike, connected_vertices};
use super::pile::Pile;

type Clique<'a> = Pile<'a, Vertex>;

pub fn explore<VertexSet, Graph, Consumer>(graph: &Graph, consumer: &mut Consumer)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Consumer: CliqueConsumer,
{
    let candidates = connected_vertices(graph);
    let num_candidates = candidates.len();
    if num_candidates > 0 {
        visit(
            graph,
            consumer,
            candidates,
            VertexSet::with_capacity(num_candidates),
            None,
        );
    }
}

fn visit<VertexSet, Graph, Consumer>(
    graph: &Graph,
    consumer: &mut Consumer,
    mut candidates: VertexSet,
    mut excluded: VertexSet,
    clique: Option<&Clique>,
) where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Consumer: CliqueConsumer,
{
    debug_assert!(candidates.all(|&v| graph.degree(v) > 0));
    debug_assert!(excluded.all(|&v| graph.degree(v) > 0));
    debug_assert!(candidates.is_disjoint(&excluded));
    debug_assert!(!candidates.is_empty());

    while let Some(v) = candidates.pop_arbitrary() {
        let neighbours = graph.neighbours(v);
        let neighbouring_candidates: VertexSet = candidates.intersection_collect(neighbours);
        if !neighbouring_candidates.is_empty() {
            visit(
                graph,
                consumer,
                neighbouring_candidates,
                excluded.intersection_collect(neighbours),
                Some(&Pile::on(clique, v)),
            );
        } else if excluded.is_disjoint(neighbours) {
            consumer.accept(Pile::on(clique, v).collect());
        }
        excluded.insert(v);
    }
}
