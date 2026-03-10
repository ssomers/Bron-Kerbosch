use super::fortified_counter::FortifiedCounter;
use super::graph::{UndirectedGraph, Vertex, VertexMap, VertexSetLike, vertices};
use super::priority_queue::Priority;
use super::priority_queue::PriorityQueue;
use std::iter::FusedIterator;

// Enumerate connected vertices in degeneracy order, skipping vertices
// whose neighbours have all been enumerated already.
// Along with the vertex picked, includes the subset of neighbours already picked.
pub fn degeneracy_iter<Graph>(graph: &Graph) -> DegeneracyOrderIter<'_, Graph>
where
    Graph: UndirectedGraph,
{
    let mut priority_per_vertex = VertexMap::new(None, graph.order());
    let mut queue = PriorityQueue::new(graph.max_degree());
    let mut left_to_pick = FortifiedCounter::new();
    for v in vertices(graph) {
        if let Some(priority) = Priority::new(graph.degree(v)) {
            priority_per_vertex[v] = Some(priority);
            queue.put(v, priority);
            left_to_pick.add(v);
        }
    }

    DegeneracyOrderIter {
        graph,
        priority_per_vertex,
        queue,
        left_to_pick,
    }
}

pub struct DegeneracyOrderIter<'a, Graph> {
    graph: &'a Graph,
    priority_per_vertex: VertexMap<Option<Priority>>,
    // If priority is None, the vertex either:
    // - was always irrelevant (unconnected);
    // - was already picked itself;
    // - had all its neighbours picked.
    queue: PriorityQueue<Vertex>,
    left_to_pick: FortifiedCounter<Vertex>,
}

impl<Graph> DegeneracyOrderIter<'_, Graph>
where
    Graph: UndirectedGraph,
{
    fn is_consistent(&self) -> bool {
        self.priority_per_vertex
            .iter()
            .all(|(v, &priority)| match priority {
                None => !self.left_to_pick.contains(v),
                Some(priority) => self.queue.contains(v, priority) && self.left_to_pick.contains(v),
            })
    }

    fn promote(&mut self, v: Vertex) {
        if let Some(old_priority) = self.priority_per_vertex[v] {
            debug_assert!(self.queue.contains(v, old_priority));
            debug_assert!(self.left_to_pick.contains(v));
            let new_priority = Priority::new(old_priority.get() - 1);
            self.priority_per_vertex[v] = new_priority;
            if let Some(new_priority) = new_priority {
                self.queue.put(v, new_priority);
            } else {
                self.left_to_pick.remove(v);
            }
        }
    }
}

impl<VertexSet, Graph> FusedIterator for DegeneracyOrderIter<'_, Graph>
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
}

impl<VertexSet, Graph> Iterator for DegeneracyOrderIter<'_, Graph>
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    type Item = (Vertex, VertexSet);

    fn next(&mut self) -> Option<Self::Item> {
        while self.left_to_pick.count() > 0 {
            debug_assert!(self.is_consistent());
            let pick = self.queue.pop().expect("Cannot pop more than was put");
            if self.priority_per_vertex[pick].take().is_some() {
                self.left_to_pick.remove(pick);
                let neighbours = self.graph.neighbours(pick);
                let neighbouring_picked = neighbours
                    .intersection_with_fn_collect(|v| self.priority_per_vertex[v].is_none());
                neighbours.for_each(|v| self.promote(v));
                return Some((pick, neighbouring_picked));
            }
        }
        None
    }
}
