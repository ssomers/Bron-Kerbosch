use super::fortified_counter::FortifiedCounter;
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
        if let Some(priority) = Priority::new(degree) {
            priority_per_vertex[v] = Some(priority);
            queue.insert(v, priority);
        }
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
    elements: FortifiedCounter<T>,
}

impl<T> PriorityQueue<T>
where
    T: Copy + Eq,
{
    fn new(max_priority: usize) -> Self {
        PriorityQueue {
            stack_per_priority: vec![vec![]; max_priority],
            elements: FortifiedCounter::new(),
        }
    }

    fn empty(&self) -> bool {
        self.elements.empty()
    }

    fn insert(&mut self, element: T, priority: Priority) {
        self.elements.add(&element);
        self.stack_per_priority[priority.get() - 1].push(element);
    }

    // Requeue with a more urgent priority or dequeue.
    // Don't bother to remove the original entry from the queue,
    // since the vertex will be skipped when popped, and thanks to
    // elements.count we might not need to pop it at all.
    fn promote(&mut self, element: T, old_priority: Priority) -> Option<Priority> {
        debug_assert!(self.elements.contains(&element));
        let new_priority = Priority::new(old_priority.get() - 1);
        if let Some(p) = new_priority {
            self.stack_per_priority[p.get() - 1].push(element);
        } else {
            self.forget(element);
        }
        new_priority
    }

    // We may return an element already popped, even though it was passed to Forget,
    // in case its priority was promoted earlier on. That's why we do not count
    // the element as picked, but wait for the caller to Forget it. The caller must
    // somehow ensure to Forget the same element only once.
    fn pop(&mut self) -> Option<T> {
        for stack in &mut self.stack_per_priority {
            if let Some(element) = stack.pop() {
                return Some(element);
            }
        }
        None
    }

    fn forget(&mut self, element: T) {
        self.elements.remove(&element);
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
                let new_priority = self.queue.promote(v, old_priority);
                self.priority_per_vertex[v] = new_priority;
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
