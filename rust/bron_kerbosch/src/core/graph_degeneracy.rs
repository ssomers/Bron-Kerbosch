use super::graph::{UndirectedGraph, Vertex, VertexMap, VertexSetLike};
use std::iter::FusedIterator;

// Enumerate connected vertices in degeneracy order, skipping vertices
// whose neighbours have all been enumerated already.
pub fn degeneracy_iter<Graph>(graph: &Graph) -> DegeneracyOrderIter<'_, Graph>
where
    Graph: UndirectedGraph,
{
    let order = graph.order();
    let mut priority_per_vertex = VertexMap::new(None, order);
    let mut queue = PriorityQueue::new(graph.max_degree());
    for i in 0..order {
        let v = Vertex::new(i);
        let degree = graph.degree(v);
        let priority = Priority::new(degree);
        priority_per_vertex[v] = priority;
        queue.insert(v, priority_per_vertex[v]);
    }

    DegeneracyOrderIter {
        graph,
        priority_per_vertex,
        queue,
    }
}

type Priority = std::num::NonZero<usize>; // = degree - #of yielded neighbours

struct PriorityQueue<T> {
    stack_per_priority: Vec<Vec<T>>,
    num_left_to_pick: usize,
}

impl<T> PriorityQueue<T>
where
    T: Copy + Eq,
{
    fn new(max_priority: usize) -> Self {
        PriorityQueue {
            stack_per_priority: vec![vec![]; max_priority],
            num_left_to_pick: 0,
        }
    }

    fn empty(&self) -> bool {
        self.num_left_to_pick == 0
    }

    fn insert(&mut self, element: T, priority: Option<Priority>) {
        if let Some(priority) = priority {
            self.stack_per_priority[priority.get() - 1].push(element);
            self.num_left_to_pick += 1
        }
    }

    fn promote(&mut self, element: T, priority: Option<Priority>) {
        if let Some(priority) = priority {
            self.stack_per_priority[priority.get() - 1].push(element);
        } else {
            self.forget(element);
        }
    }

    fn pop(&mut self) -> Option<T> {
        for stack in &mut self.stack_per_priority {
            if let Some(element) = stack.pop() {
                return Some(element);
            }
        }
        None
    }

    fn forget(&mut self, _element: T) {
        self.num_left_to_pick -= 1
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

    fn reassess<VertexSet: VertexSetLike>(&mut self, neighbours: &VertexSet) {
        neighbours.for_each(|v| {
            if let Some(old_priority) = self.priority_per_vertex[v] {
                // Requeue with a more urgent priority or dequeue.
                // Don't bother to remove the original entry from the queue,
                // since the vertex will be skipped when popped, and thanks to
                // num_left_to_pick we might not need to pop it at all.
                let new_priority = Priority::new(old_priority.get() - 1);
                self.priority_per_vertex[v] = new_priority;
                self.queue.promote(v, new_priority);
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
        while !self.queue.empty() {
            debug_assert!(self.is_consistent());
            let pick = self.queue.pop().expect("Cannot pop more than has been put");
            if self.priority_per_vertex[pick].is_some() {
                self.priority_per_vertex[pick] = None;
                self.queue.forget(pick);
                self.reassess(self.graph.neighbours(pick));
                return Some(pick);
            }
        }
        None
    }
}
