use super::fortified_counter::FortifiedCounter;
use super::graph::Graph;
use super::priority_queue::Priority;
use super::priority_queue::PriorityQueue;
use super::vertex::{Vertex, VertexMap};
use super::vertexsetlike::VertexSetLike;
use std::ops::Not;

// Enumerate connected vertices in degeneracy order, skipping vertices
// whose neighbours have all been enumerated already.
// Along with the vertex picked, includes the subset of neighbours already picked.
pub struct Degeneracy<'a, VertexSet>
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

impl<'a, VertexSet> Degeneracy<'a, VertexSet>
where
    VertexSet: VertexSetLike,
{
    pub fn on(graph: &'a Graph<VertexSet>) -> Self {
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

        Self {
            graph,
            left_to_pick,
            priority_per_vertex,
            queue,
        }
    }

    fn is_consistent(&self) -> bool {
        self.priority_per_vertex
            .iter()
            .all(|(v, &priority)| match priority {
                None => self.left_to_pick.contains(v).not(),
                Some(priority) => self.left_to_pick.contains(v) && self.queue.contains(priority, v),
            })
    }

    fn promote_neighbours(&mut self, pick: Vertex) {
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
                    self.left_to_pick.remove(v);
                }
            }
        });
    }

    pub fn apply(mut self, mut action: impl FnMut(Vertex, DegeneracyAttorney<'_, 'a, VertexSet>)) {
        debug_assert!(self.is_consistent());
        while self.left_to_pick.count() > 0 {
            let pick = self.queue.pop().expect("Cannot pop more than was put");
            if self.priority_per_vertex[pick].take().is_some() {
                action(pick, DegeneracyAttorney(&self));
                self.left_to_pick.remove(pick);
                self.promote_neighbours(pick);
            }
            debug_assert!(self.is_consistent());
        }
    }
}

pub struct DegeneracyAttorney<'b, 'a, VertexSet: VertexSetLike>(&'b Degeneracy<'a, VertexSet>);

impl<'b, 'a, VertexSet: VertexSetLike> DegeneracyAttorney<'b, 'a, VertexSet> {
    pub fn is_candidate(&'b self, v: Vertex) -> bool {
        self.0.priority_per_vertex[v].is_some()
    }

    pub fn partition_neighbours(&'b self, v: Vertex) -> (VertexSet, VertexSet) {
        let neighbours = self.0.graph.neighbours(v);
        let n = neighbours.len();
        assert!(n > 0);
        let mut neighbouring_candidates = VertexSet::with_capacity(n);
        let mut neighbouring_excluded = VertexSet::with_capacity(n - 1);
        neighbours.for_each(|v| {
            if self.is_candidate(v) {
                neighbouring_candidates.insert(v);
            } else {
                neighbouring_excluded.insert(v);
            }
        });
        debug_assert!(neighbouring_candidates.is_empty().not());
        (neighbouring_candidates, neighbouring_excluded)
    }
}
