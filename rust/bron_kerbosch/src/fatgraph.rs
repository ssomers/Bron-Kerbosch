use graph::{assert_adjacencies, Adjacencies, NewableUndirectedGraph, UndirectedGraph, Vertex};

use std::collections::HashSet;

#[derive(Debug)]
pub struct VertexRec {
    first_arc_out: Option<ArcRec>,
    degree: usize,
    /*
    index: usize,
    */
}

/*
#[derive(Debug)]
pub struct Edge {
    index: usize,
    vtx_a: &VertexRec,
    vtx_b: &VertexRec,
    fwd_arc: ArcRec,
    rev_arc: ArcRec,
}
*/

#[derive(Debug)]
pub struct ArcRec {
    vtx_a: Vertex,
    vtx_b: Vertex,
    next_arc_out: Option<Box<ArcRec>>,
}

#[derive(Debug)]
pub struct FatUndirectedGraph {
    vertices: Vec<VertexRec>,
}

struct NeighbourIterator<'a> {
    degree: usize,
    arc_out: Option<&'a ArcRec>,
}
impl<'a> NeighbourIterator<'a> {
    pub fn new(g: &'a FatUndirectedGraph, node: Vertex) -> NeighbourIterator<'a> {
        NeighbourIterator {
            degree: g.vertices[node as usize].degree,
            arc_out: g.vertices[node as usize].first_arc_out.as_ref(),
        }
    }
}

impl<'a> Iterator for NeighbourIterator<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {
        if let Some(a) = self.arc_out {
            self.arc_out = a.next_arc_out.as_ref().map(|a| &**a);
            Some(a.vtx_b)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.degree, Some(self.degree))
    }
}

impl UndirectedGraph for FatUndirectedGraph {
    fn order(&self) -> u32 {
        self.vertices.len() as u32
    }

    fn size(&self) -> u32 {
        let total: usize = self.vertices.iter().map(|v| v.degree).sum();
        assert!(total % 2 == 0);
        (total / 2) as u32
    }

    fn degree(&self, node: Vertex) -> u32 {
        self.vertices[node as usize].degree as u32
    }

    fn neighbour_difference(&self, candidates: &HashSet<Vertex>, node: Vertex) -> Vec<Vertex> {
        let neighbours: HashSet<Vertex> = NeighbourIterator::new(self, node).collect();
        candidates.difference(&neighbours).cloned().collect()
    }

    fn neighbour_intersection(&self, set: &HashSet<Vertex>, node: Vertex) -> HashSet<Vertex> {
        NeighbourIterator::new(self, node)
            .filter(|v| set.contains(v))
            .collect()
    }

    fn neighbour_intersection_count(&self, set: &HashSet<Vertex>, node: Vertex) -> usize {
        NeighbourIterator::new(self, node)
            .filter(|v| set.contains(v))
            .count()
    }

    fn visit_neighbours<F>(&self, node: Vertex, mut f: F)
    where
        F: FnMut(Vertex),
    {
        for v in NeighbourIterator::new(self, node) {
            f(v);
        }
    }
}

impl NewableUndirectedGraph for FatUndirectedGraph {
    fn new(adjacencies: Adjacencies) -> Self {
        debug_assert!(assert_adjacencies(&adjacencies));
        let mut vertices: Vec<VertexRec> = vec![];
        for (v, neighbours) in adjacencies.into_iter().enumerate() {
            let mut next_arc: Option<ArcRec> = None;
            for &n in neighbours.iter() {
                let arc = ArcRec {
                    vtx_a: v as Vertex,
                    vtx_b: n,
                    next_arc_out: next_arc.map(|a| Box::new(a)),
                };
                next_arc = Some(arc);
            }
            vertices.push(VertexRec {
                first_arc_out: next_arc,
                degree: neighbours.len(),
            });
        }

        /*
        let arcs: Vec<ArcRec> = adjacencies
            .iter()
            .enumerate()
            .flat_map(|(v, ref neighbours)| {
                neighbours
                    .iter()
                    .map(|&n| ArcRec {
                        vtx_a: v as Vertex,
                        vtx_b: n,
                        next_arc_out: None,
                    })
                    .collect::<Vec<ArcRec>>()
            })
            .collect();
        let vertices = adjacencies
            .into_iter()
            .map(|neighbours| VertexRec {
                first_arc_out: None,
                neighbours,
            })
            .collect();
        */
        FatUndirectedGraph { vertices }
    }
}
