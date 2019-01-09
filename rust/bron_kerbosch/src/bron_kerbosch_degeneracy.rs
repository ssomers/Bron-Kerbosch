//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use graph::{UndirectedGraph, Vertex};

use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug)]
struct PriorityQueue<T> {
    stack_per_priority: Vec<Vec<T>>,
}

impl<T> PriorityQueue<T>
where
    T: Copy + Eq,
{
    fn new(max_priority: usize) -> Self {
        PriorityQueue {
            stack_per_priority: vec![vec![]; max_priority + 1],
        }
    }

    fn put(&mut self, priority: usize, element: T) {
        self.stack_per_priority[priority].push(element);
    }

    fn pop(&mut self) -> Option<T> {
        for stack in &mut self.stack_per_priority {
            match stack.pop() {
                Some(element) => return Some(element),
                None => continue,
            }
        }
        None
    }

    fn contains(&self, priority: usize, element: T) -> bool {
        self.stack_per_priority[priority].contains(&element)
    }
}

type Priority = u32;

#[derive(Debug)]
pub struct DegeneracyOrderIter<'a> {
    graph: &'a UndirectedGraph,
    no_priority: Priority, // some number distinct from any degree or decrement thereof
    priority_per_node: Vec<Priority>,
    // If priority is no_priority, node was already picked or was always irrelevant (unconnected);
    // otherwise, node is still queued and priority = degree - number of picked neighbours.
    queue: PriorityQueue<Vertex>,
    num_left_to_pick: usize,
}

impl<'a> DegeneracyOrderIter<'a> {
    fn pick_with_lowest_degree(&mut self) -> Vertex {
        debug_assert!(self
            .priority_per_node
            .iter()
            .enumerate()
            .all(|(v, &d)| d == self.no_priority || self.queue.contains(d as usize, v as Vertex)));
        loop {
            let v = self.queue.pop().expect("Cannot pop more than has been put");
            if self.priority_per_node[v as usize] != self.no_priority {
                self.priority_per_node[v as usize] = self.no_priority;
                break v;
            }
            // else v was requeued with a more urgent priority and therefore already picked
        }
    }
}

impl<'a> Iterator for DegeneracyOrderIter<'a> {
    type Item = Vertex;
    fn next(&mut self) -> Option<Vertex> {
        if self.num_left_to_pick == 0 {
            None
        } else {
            self.num_left_to_pick -= 1;
            let i = self.pick_with_lowest_degree();
            for &v in self.graph.adjacencies(i) {
                let old_priority = self.priority_per_node[v as usize];
                if old_priority != self.no_priority {
                    debug_assert!(old_priority > 0);
                    let new_priority = old_priority - 1;
                    debug_assert_ne!(new_priority, self.no_priority);
                    // Requeue with a more urgent priority, but don't bother to remove
                    // the original entry - it will be skipped if it's reached at all.
                    self.priority_per_node[v as usize] = new_priority;
                    self.queue.put(new_priority as usize, v);
                }
            }
            Some(i)
        }
    }
}

pub fn degeneracy_order_smart<'a>(
    graph: &'a UndirectedGraph,
    candidates: &HashSet<Vertex>,
) -> DegeneracyOrderIter<'a> {
    let order = graph.order();
    let no_priority = order;
    let mut max_priority: Priority = 0;
    let mut priority_per_node: Vec<Priority> = vec![no_priority; order as usize];
    for &c in candidates {
        let priority = graph.degree(c);
        debug_assert_ne!(priority, no_priority);
        max_priority = max(max_priority, priority);
        priority_per_node[c as usize] = priority;
    }
    let mut queue: PriorityQueue<Vertex> = PriorityQueue::new(max_priority as usize);
    for &c in candidates {
        queue.put(priority_per_node[c as usize] as usize, c);
    }
    DegeneracyOrderIter {
        graph,
        no_priority,
        priority_per_node,
        queue,
        num_left_to_pick: candidates.len(),
    }
}
