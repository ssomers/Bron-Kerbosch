use super::graph::{UndirectedGraph, Vertex, VertexMap, VertexSetLike, vertices};
use std::iter::FusedIterator;

// Enumerate connected vertices in degeneracy order, skipping vertices
// whose neighbours have all been enumerated already.
pub fn degeneracy_iter<Graph>(graph: &Graph) -> DegeneracyOrderIter<'_, Graph>
where
    Graph: UndirectedGraph,
{
    let order = graph.order();
    let mut priority_per_vertex = VertexMap::new(None, order);
    let mut max_priority: usize = 0;
    let mut num_candidates: usize = 0;
    for v in vertices(graph) {
        let degree = graph.degree(v);
        if let Some(priority) = Priority::new(degree) {
            priority_per_vertex[v] = Some(priority);
            max_priority = max_priority.max(priority.get());
            num_candidates += 1;
        }
    }
    let mut queue = PriorityQueue::new(max_priority);
    for v in vertices(graph) {
        if let Some(priority) = priority_per_vertex[v] {
            queue.put(priority, v);
        }
    }

    DegeneracyOrderIter {
        graph,
        priority_per_vertex,
        queue,
        num_left_to_pick: num_candidates,
    }
}

type Priority = std::num::NonZero<usize>; // = degree - #of yielded neighbours

struct PriorityQueue<T> {
    stack_per_priority: Vec<Vec<T>>,
}

impl<T> PriorityQueue<T>
where
    T: Copy + Eq,
{
    fn new(max_priority: usize) -> Self {
        PriorityQueue {
            stack_per_priority: vec![vec![]; max_priority],
        }
    }

    fn put(&mut self, priority: Priority, element: T) {
        self.stack_per_priority[priority.get() - 1].push(element);
    }

    fn pop(&mut self) -> Option<T> {
        for stack in &mut self.stack_per_priority {
            if let Some(element) = stack.pop() {
                return Some(element);
            }
        }
        None
    }

    fn contains(&self, priority: Priority, element: T) -> bool {
        if !(cfg!(debug_assertions)) {
            panic!("not suitable for use in release code")
        }
        self.stack_per_priority[priority.get() - 1].contains(&element)
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
    num_left_to_pick: usize,
}

impl<Graph> DegeneracyOrderIter<'_, Graph> {
    fn is_consistent(&self) -> bool {
        self.priority_per_vertex
            .iter()
            .all(|(v, &priority)| match priority {
                None => true,
                Some(priority) => self.queue.contains(priority, v),
            })
    }

    fn requeue<VertexSet: VertexSetLike>(&mut self, neighbours: &VertexSet) {
        neighbours.for_each(|v| {
            match self.priority_per_vertex[v] {
                None => {}
                Some(old_priority) => {
                    // Requeue with a more urgent priority or unqueue.
                    // Don't bother to remove the original entry from the queue,
                    // since the vertex will be skipped when popped, and thanks to
                    // num_left_to_pick we might not need to pop it at all.
                    let new_priority = Priority::new(old_priority.get() - 1);
                    self.priority_per_vertex[v] = new_priority;
                    if let Some(new_priority) = new_priority {
                        self.queue.put(new_priority, v);
                    } else {
                        self.num_left_to_pick -= 1;
                    }
                }
            }
        });
    }
}

impl<Graph> FusedIterator for DegeneracyOrderIter<'_, Graph> where Graph: UndirectedGraph {}
impl<Graph> Iterator for DegeneracyOrderIter<'_, Graph>
where
    Graph: UndirectedGraph,
{
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {
        while self.num_left_to_pick > 0 {
            debug_assert!(self.is_consistent());
            let pick = self.queue.pop().expect("Cannot pop more than has been put");
            if self.priority_per_vertex[pick].is_some() {
                self.priority_per_vertex[pick] = None;
                self.num_left_to_pick -= 1;
                self.requeue(self.graph.neighbours(pick));
                return Some(pick);
            }
        }
        None
    }
}
