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
    let mut left_to_pick = FortifiedCounter::new();
    let mut priority_per_vertex = VertexMap::new(None, graph.order());
    let mut queue = PriorityQueue::new(graph.max_degree());
    for v in vertices(graph) {
        if let Some(priority) = Priority::new(graph.degree(v)) {
            left_to_pick.add(v);
            priority_per_vertex[v] = Some(priority);
            queue.put(v, priority);
        }
    }

    DegeneracyOrderIter {
        graph,
        left_to_pick,
        priority_per_vertex,
        queue,
    }
}

pub struct DegeneracyOrderIter<'a, Graph> {
    graph: &'a Graph,
    left_to_pick: FortifiedCounter<Vertex>,
    priority_per_vertex: VertexMap<Option<Priority>>,
    // If priority is None, the vertex either:
    // - was always irrelevant (unconnected);
    // - was already picked itself;
    // - had all its neighbours picked.
    queue: PriorityQueue<Vertex>,
}

impl<VertexSet, Graph> DegeneracyOrderIter<'_, Graph>
where
    VertexSet: VertexSetLike,
    Graph: UndirectedGraph<VertexSet = VertexSet>,
{
    fn is_consistent(&self) -> bool {
        self.priority_per_vertex
            .iter()
            .all(|(v, &priority)| match priority {
                None => !self.left_to_pick.contains(v),
                Some(priority) => self.left_to_pick.contains(v) && self.queue.contains(v, priority),
            })
    }

    fn evaluate_neighbours(&mut self, pick: Vertex) -> VertexSet {
        let mut picked_neighbours = VertexSet::new();
        self.graph.neighbours(pick).for_each(|v| {
            if let Some(old_priority) = self.priority_per_vertex[v] {
                debug_assert!(self.left_to_pick.contains(v));
                debug_assert!(self.queue.contains(v, old_priority));
                let new_priority = Priority::new(old_priority.get() - 1);
                self.priority_per_vertex[v] = new_priority;
                if let Some(new_priority) = new_priority {
                    self.queue.put(v, new_priority);
                } else {
                    self.left_to_pick.remove(v);
                }
            } else {
                picked_neighbours.insert(v);
            }
        });
        picked_neighbours
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
                let picked_neighbours = self.evaluate_neighbours(pick);
                return Some((pick, picked_neighbours));
            }
        }
        None
    }
}
