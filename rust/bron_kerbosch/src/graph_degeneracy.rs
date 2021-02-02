use graph::{UndirectedGraph, Vertex, VertexMap, VertexSetLike};

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
                Some(max_priority) => vec![vec![]; max_priority.get() as usize],
            },
        }
    }

    fn put(&mut self, priority: Priority, element: T) {
        self.stack_per_priority[priority.get() as usize - 1].push(element);
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
        self.stack_per_priority[priority.get() as usize - 1].contains(&element)
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

impl<'a, Graph> DegeneracyOrderIter<'a, Graph> {
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

impl<'a, Graph> Iterator for DegeneracyOrderIter<'a, Graph>
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

#[cfg(all(test, not(miri)))]
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
                &(2..99usize).prop_flat_map(|order| {
                    proptest::collection::vec(
                        proptest::collection::btree_set(0..order - 1, ..order),
                        order,
                    )
                }),
                |adjac| {
                    let order = adjac.len();
                    let adjacencies: Vec<BTreeSet<Vertex>> =
                        (0..order).map(|_| BTreeSet::new()).collect();
                    let mut adjacencies = Adjacencies::sneak_in(adjacencies);
                    for (v, adjacent_to_v) in adjac.iter().enumerate() {
                        let v = Vertex::new(v);
                        for &w in adjacent_to_v {
                            let w = Vertex::new(w);
                            if w != v {
                                adjacencies[v].insert(w);
                                adjacencies[w].insert(v);
                            }
                        }
                    }

                    let g = SlimUndirectedGraph::new(adjacencies);
                    let ordering: Vec<Vertex> = degeneracy_ordering(&g, 0).collect();
                    let orderin: Vec<Vertex> = degeneracy_ordering(&g, -1).collect();
                    let ordering_set: BTreeSet<Vertex> = ordering.iter().copied().collect();
                    let orderin_set: BTreeSet<Vertex> = orderin.iter().copied().collect();
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
