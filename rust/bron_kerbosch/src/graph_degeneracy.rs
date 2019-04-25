//! Bron-Kerbosch algorithm with pivot and degeneracy ordering, optimized

use graph::{UndirectedGraph, Vertex, VertexSetLike};

pub fn degeneracy_ordering<'a, VertexSet>(
    graph: &'a UndirectedGraph<VertexSet>,
    drop: isize,
) -> DegeneracyOrderIter<'a, VertexSet>
where
    VertexSet: VertexSetLike,
{
    debug_assert!(drop <= 0);
    let order = graph.order();
    let mut max_priority: Option<Priority> = None;
    let mut priority_per_vertex: Vec<Option<Priority>> = vec![None; order as usize];
    let mut num_candidates: isize = 0;
    for c in 0..order {
        let degree = graph.degree(c);
        if degree > 0 {
            let priority = Priority::new(degree + 1);
            max_priority = max_priority.iter().cloned().chain(priority).max();
            debug_assert!(max_priority.is_some());
            priority_per_vertex[c as usize] = priority;
            num_candidates += 1;
        }
    }
    let mut queue: PriorityQueue<Vertex> = PriorityQueue::new(max_priority);
    for c in 0..order {
        if let Some(priority) = priority_per_vertex[c as usize] {
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

type Priority = std::num::NonZeroU32;

#[derive(Debug)]
struct PriorityQueue<T> {
    queue_per_priority: Vec<Vec<T>>,
}

impl<T> PriorityQueue<T>
where
    T: Copy + Eq,
{
    fn new(max_priority: Option<Priority>) -> Self {
        let max_priority_val = max_priority.map_or(0, |p| p.get());
        PriorityQueue {
            queue_per_priority: vec![vec![]; max_priority_val as usize],
        }
    }

    fn put(&mut self, priority: Priority, element: T) {
        self.queue_per_priority[priority.get() as usize - 1].push(element);
    }

    fn pop(&mut self) -> Option<T> {
        for queue in &mut self.queue_per_priority {
            if let Some(element) = queue.pop() {
                return Some(element);
            }
        }
        None
    }

    #[cfg(debug_assertions)]
    fn contains(&self, priority: Priority, element: T) -> bool {
        self.queue_per_priority[priority.get() as usize - 1].contains(&element)
    }
    #[cfg(not(debug_assertions))]
    fn contains(&self, _: Priority, _: T) -> bool {
        panic!("don't come here")
    }
}
pub struct DegeneracyOrderIter<'a, VertexSet> {
    graph: &'a UndirectedGraph<VertexSet>,
    priority_per_vertex: Vec<Option<Priority>>,
    // If priority is None, vertex was already picked or was always irrelevant (unconnected);
    // otherwise, vertex is still queued and priority = degree - number of picked neighbours +1.
    // +1 because we want the priority number to be NonZero to allow free wrapping inside Option.
    queue: PriorityQueue<Vertex>,
    num_left_to_pick: isize,
}

impl<'a, VertexSet> DegeneracyOrderIter<'a, VertexSet> {
    fn pick_with_lowest_degree(&mut self) -> Vertex {
        debug_assert!(self
            .priority_per_vertex
            .iter()
            .enumerate()
            .all(|(v, &p)| match p {
                None => true, // might still be in some queue
                Some(p) => self.queue.contains(p, v as Vertex),
            }));
        loop {
            let v = self.queue.pop().expect("Cannot pop more than has been put");
            if self.priority_per_vertex[v as usize].is_some() {
                self.priority_per_vertex[v as usize] = None;
                return v;
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
                if let Some(old_priority) = self.priority_per_vertex[v as usize] {
                    // Since this is an unvisited neighbour of a vertex just being picked,
                    // its priority can't be down to the minimum.
                    let new_priority = Priority::new(old_priority.get() - 1);
                    debug_assert!(new_priority.is_some());
                    // Requeue with a more urgent priority, but don't bother to remove
                    // the original entry - it will be skipped if it's reached at all.
                    self.priority_per_vertex[v as usize] = new_priority;
                    self.queue.put(new_priority.unwrap(), v);
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
