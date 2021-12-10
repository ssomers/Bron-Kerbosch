//! Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

use crate::bron_kerbosch_pivot::{visit, PivotChoice};
use crate::graph::{UndirectedGraph, Vertex, VertexSetLike};
use crate::pile::Pile;
use crate::reporter::Reporter;

pub fn explore<VertexSet, Graph, Rprtr>(graph: &Graph, reporter: &mut Rprtr)
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
    Rprtr: Reporter,
{
    let order = graph.order();
    if let Some(pivot) = (0..order).map(Vertex::new).max_by_key(|&v| graph.degree(v)) {
        let mut excluded = Graph::VertexSet::with_capacity(order);
        for v in (0..order).map(Vertex::new) {
            let neighbours = graph.neighbours(v);
            if !neighbours.is_empty() && !neighbours.contains(pivot) {
                let neighbouring_excluded: VertexSet = neighbours.intersection_collect(&excluded);
                if neighbouring_excluded.len() < neighbours.len() {
                    let neighbouring_candidates: VertexSet =
                        neighbours.difference_collect(&neighbouring_excluded);
                    visit(
                        graph,
                        reporter,
                        PivotChoice::MaxDegreeLocalX,
                        neighbouring_candidates,
                        neighbouring_excluded,
                        Some(&Pile::from(v)),
                    );
                }
                excluded.insert(v);
            }
        }
    }
}
