//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use graph::{UndirectedGraph, Vertex, VertexSetLike};

use std::cmp::max;

pub fn degeneracy_ordering<'a, VertexSet>(
    graph: &'a UndirectedGraph<VertexSet>,
    drop: isize,
) -> DegeneracyOrderIter<'a, VertexSet>
where
    VertexSet: VertexSetLike,
{
    debug_assert!(drop <= 0);
    let order = graph.order();
    let no_priority = order;
    let mut max_priority: Priority = 0;
    let mut num_candidates: isize = 0;
    let mut priority_per_node: Vec<Priority> = vec![no_priority; order as usize];
    for c in 0..order {
        let degree = graph.degree(c);
        if degree > 0 {
            let priority = degree;
            debug_assert_ne!(priority, no_priority);
            num_candidates += 1;
            max_priority = max(max_priority, priority);
            priority_per_node[c as usize] = priority;
        }
    }
    let mut queue: PriorityQueue<Vertex> = PriorityQueue::new(max_priority as usize);
    for c in 0..order {
        let priority = priority_per_node[c as usize];
        if priority != no_priority {
            queue.put(priority as usize, c);
        }
    }

    DegeneracyOrderIter {
        graph,
        no_priority,
        priority_per_node,
        queue,
        num_left_to_pick: num_candidates + drop,
    }
}

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

pub struct DegeneracyOrderIter<'a, VertexSet> {
    graph: &'a UndirectedGraph<VertexSet>,
    no_priority: Priority, // some number distinct from any degree or decrement thereof
    priority_per_node: Vec<Priority>,
    // If priority is no_priority, node was already picked or was always irrelevant (unconnected);
    // otherwise, node is still queued and priority = degree - number of picked neighbours.
    queue: PriorityQueue<Vertex>,
    num_left_to_pick: isize,
}

impl<'a, VertexSet> DegeneracyOrderIter<'a, VertexSet> {
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

impl<'a, VertexSet> Iterator for DegeneracyOrderIter<'a, VertexSet>
where
    VertexSet: VertexSetLike,
{
    type Item = Vertex;
    fn next(&mut self) -> Option<Vertex> {
        if self.num_left_to_pick <= 0 {
            None
        } else {
            self.num_left_to_pick -= 1;
            let i = self.pick_with_lowest_degree();
            self.graph.neighbours(i).for_each(|v| {
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
            });
            Some(i)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate proptest;
    use self::proptest::prelude::*;
    use self::proptest::test_runner::TestRunner;
    use super::*;
    use graph::{connected_vertices, Adjacencies, NewableUndirectedGraph};
    use slimgraph::SlimUndirectedGraph;
    use std::collections::BTreeSet;

    #[test]
    fn test_degeneracy_order() {
        TestRunner::default()
            .run(
                &(2..99u32).prop_flat_map(|order| {
                    proptest::collection::vec(
                        proptest::collection::btree_set(0..order - 1, ..order as usize),
                        order as usize,
                    )
                }),
                |adjac| {
                    let order = adjac.len();
                    let mut adjacencies: Adjacencies<BTreeSet<Vertex>> =
                        (0..order).map(|_| BTreeSet::new()).collect();
                    for (v, adjacent_to_v) in adjac
                        .iter()
                        .enumerate()
                        .map(|(i, neighbours)| (i as Vertex, neighbours))
                    {
                        for &w in adjacent_to_v {
                            if w != v {
                                adjacencies[v as usize].insert(w);
                                adjacencies[w as usize].insert(v);
                            }
                        }
                    }

                    let g = SlimUndirectedGraph::new(adjacencies);
                    let ordering: Vec<Vertex> = degeneracy_ordering(&g, 0).collect();
                    let orderin: Vec<Vertex> = degeneracy_ordering(&g, -1).collect();
                    let ordering_set: BTreeSet<Vertex> = ordering.iter().cloned().collect();
                    let orderin_set: BTreeSet<Vertex> = orderin.iter().cloned().collect();
                    assert_eq!(ordering.len(), ordering_set.len());
                    assert_eq!(orderin.len(), orderin_set.len());
                    assert_eq!(orderin.len(), ordering.len().saturating_sub(1));
                    assert_eq!(ordering_set, connected_vertices(&g));
                    assert!(orderin_set.is_subset(&ordering_set));
                    Ok(())
                },
            )
            .unwrap();
    }
}