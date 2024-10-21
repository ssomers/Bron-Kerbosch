use super::graph::{UndirectedGraph, Vertex, VertexMap, VertexSetLike};
use std::iter::FusedIterator;

pub fn degeneracy_ordering<Graph>(graph: &Graph, drop: isize) -> DegeneracyOrderIter<Graph>
where
    Graph: UndirectedGraph,
{
    debug_assert!(drop <= 0);
    let order = graph.order();
    let mut priority_per_vertex: VertexMap<Option<Priority>> = VertexMap::new(None, order);
    let mut max_priority: Option<Priority> = None;
    let mut num_candidates: isize = 0;
    for c in 0..order {
        let c = Vertex::new(c);
        let degree = graph.degree(c);
        if degree > 0 {
            let priority = Priority::new(degree + 1);
            priority_per_vertex[c] = priority;
            max_priority = max_priority.iter().copied().chain(priority).max();
            debug_assert!(max_priority.is_some());
            num_candidates += 1;
        }
    }
    let mut queue = PriorityQueue::new(max_priority);
    for c in 0..order {
        let c = Vertex::new(c);
        if let Some(priority) = priority_per_vertex[c] {
            queue.put(priority, c);
        }
    }

    DegeneracyOrderIter {
        graph,
        priority_per_vertex,
        queue,
        num_left_to_pick: num_candidates + drop,
    }
}

type Priority = std::num::NonZeroUsize;

struct PriorityQueue<T> {
    stack_per_priority: Vec<Vec<T>>,
}

impl<T> PriorityQueue<T>
where
    T: Copy + Eq,
{
    fn new(max_priority: Option<Priority>) -> Self {
        PriorityQueue {
            stack_per_priority: match max_priority {
                None => vec![],
                Some(max_priority) => vec![vec![]; max_priority.get()],
            },
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

    #[allow(clippy::nonminimal_bool)]
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
    // If priority is None, vertex was already picked or was always irrelevant (unconnected);
    // otherwise, vertex is still queued and priority = degree - number of picked neighbours +1.
    // +1 because we want the priority number to be NonZero to allow free wrapping inside Option.
    queue: PriorityQueue<Vertex>,
    num_left_to_pick: isize,
}

impl<Graph> DegeneracyOrderIter<'_, Graph> {
    fn pick_with_lowest_degree(&mut self) -> Vertex {
        debug_assert!(self.priority_per_vertex.iter().all(|(v, &p)| match p {
            None => true, // might still be in some stack
            Some(p) => self.queue.contains(p, v),
        }));
        loop {
            let v = self.queue.pop().expect("Cannot pop more than has been put");
            if self.priority_per_vertex[v].is_some() {
                self.priority_per_vertex[v] = None;
                return v;
            }
            // else v was requeued with a more urgent priority and therefore already picked
        }
    }
}

impl<Graph> FusedIterator for DegeneracyOrderIter<'_, Graph> where Graph: UndirectedGraph {}
impl<Graph> Iterator for DegeneracyOrderIter<'_, Graph>
where
    Graph: UndirectedGraph,
{
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {
        if self.num_left_to_pick > 0 {
            self.num_left_to_pick -= 1;
            let i = self.pick_with_lowest_degree();
            self.graph.neighbours(i).for_each(|v| {
                if let Some(old_priority) = self.priority_per_vertex[v] {
                    // Since this is an unvisited neighbour of a vertex just being picked,
                    // its priority can't be down to the minimum.
                    let new_priority = Priority::new(old_priority.get() - 1);
                    debug_assert!(new_priority.is_some());
                    // Requeue with a more urgent priority, but don't bother to remove
                    // the original entry - it will be skipped if it's reached at all.
                    self.priority_per_vertex[v] = new_priority;
                    self.queue.put(new_priority.unwrap(), v);
                }
            });
            Some(i)
        } else {
            None
        }
    }
}
