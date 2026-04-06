use super::fortified_counter::FortifiedCounter;
use super::graph::Graph;
use super::priority_queue::Priority;
use super::priority_queue::PriorityQueue;
use super::vertex::{Vertex, VertexMap};
use super::vertexsetlike::VertexSetLike;
use std::iter::FusedIterator;
use std::ops::Not;

// Enumerate connected vertices in degeneracy order, skipping vertices
// whose neighbours have all been enumerated already.
// Along with the vertex picked, includes the subset of neighbours already picked.
pub fn degeneracy_iter<VertexSet>(graph: &Graph<VertexSet>) -> DegeneracyOrderIter<'_, VertexSet>
where
    VertexSet: VertexSetLike,
{
    let mut left_to_pick = FortifiedCounter::new();
    let mut priority_per_vertex = VertexMap::new(None, graph.order());
    let mut queue = PriorityQueue::new(graph.max_degree());
    for v in graph.vertices() {
        if let Some(priority) = Priority::new(graph.degree(v)) {
            left_to_pick.add(v);
            priority_per_vertex[v] = Some(priority);
            queue.put(priority, v);
        }
    }

    DegeneracyOrderIter {
        graph,
        left_to_pick,
        priority_per_vertex,
        queue,
    }
}

pub struct DegeneracyOrderIter<'a, VertexSet>
where
    VertexSet: VertexSetLike,
{
    graph: &'a Graph<VertexSet>,
    left_to_pick: FortifiedCounter<Vertex>,
    priority_per_vertex: VertexMap<Option<Priority>>,
    // If priority is None, the vertex either:
    // - was always irrelevant (unconnected);
    // - was already picked itself;
    // - had all its neighbours picked.
    queue: PriorityQueue<Vertex>,
}

impl<VertexSet> DegeneracyOrderIter<'_, VertexSet>
where
    VertexSet: VertexSetLike,
{
    fn is_consistent(&self) -> bool {
        self.priority_per_vertex
            .iter()
            .all(|(v, &priority)| match priority {
                None => self.left_to_pick.contains(v).not(),
                Some(priority) => self.left_to_pick.contains(v) && self.queue.contains(priority, v),
            })
    }

    fn evaluate_neighbours(&mut self, pick: Vertex) -> VertexSet {
        let mut picked_neighbours = VertexSet::new();
        self.graph.neighbours(pick).for_each(|v| {
            if let Some(old_priority) = self.priority_per_vertex[v] {
                debug_assert!(self.left_to_pick.contains(v));
                debug_assert!(self.queue.contains(old_priority, v));
                let new_priority = Priority::new(old_priority.get() - 1);
                // Because either the new priority takes precedence or the vertex is dequeued,
                // we don't need to find and remove the original entry in the queue.
                // We'll just skip the vertex when the old entry gets popped, and thanks to
                // left_to_pick, we might not even get to popping it at all.
                self.priority_per_vertex[v] = new_priority;
                if let Some(new_priority) = new_priority {
                    self.queue.put(new_priority, v);
                } else {
                    // We discount this neighbour already, but logically it will
                    // be (silently) picked only after we yield the current pick.
                    // So it does not belong in the current picked_neighbours.
                    self.left_to_pick.remove(v);
                }
            } else {
                picked_neighbours.insert(v);
            }
        });
        picked_neighbours
    }
}

impl<VertexSet> FusedIterator for DegeneracyOrderIter<'_, VertexSet> where VertexSet: VertexSetLike {}

impl<VertexSet> Iterator for DegeneracyOrderIter<'_, VertexSet>
where
    VertexSet: VertexSetLike,
{
    type Item = (Vertex, VertexSet);

    fn next(&mut self) -> Option<Self::Item> {
        while self.left_to_pick.count() > 0 {
            debug_assert!(self.is_consistent());
            let pick = self.queue.pop().expect("Cannot pop more than was put");
            if self.priority_per_vertex[pick].take().is_some() {
                self.left_to_pick.remove(pick);
                let picked_neighbours = self.evaluate_neighbours(pick);
                debug_assert!(picked_neighbours.len() < self.graph.degree(pick));
                return Some((pick, picked_neighbours));
            }
        }
        None
    }
}
