use graph::{assert_adjacencies, Adjacencies, NewableUndirectedGraph, UndirectedGraph, Vertex};

use std::collections::HashSet;

#[derive(Debug)]
pub struct VertexRec {
    neighbours: HashSet<Vertex>,
    /*
    first_arc_out: &Arc,
    index: usize,
    degree: u32,
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

#[derive(Debug)]
pub struct Arc {
    edge: &Edge,
    next_arc_out: &Arc,
}
*/

#[derive(Debug)]
pub struct FatUndirectedGraph {
    vertices: Vec<VertexRec>,
}

impl UndirectedGraph for FatUndirectedGraph {
    fn order(&self) -> u32 {
        self.vertices.len() as u32
    }

    fn size(&self) -> u32 {
        let total: u32 = self
            .vertices
            .iter()
            .map(|v| v.neighbours.len() as u32)
            .sum();
        assert!(total % 2 == 0);
        total / 2
    }

    fn degree(&self, node: Vertex) -> u32 {
        self.adjacencies(node).len() as u32
    }

    fn adjacencies(&self, node: Vertex) -> &HashSet<Vertex> {
        &self.vertices[node as usize].neighbours
    }
}

impl NewableUndirectedGraph for FatUndirectedGraph {
    fn new(adjacencies: Adjacencies) -> Self {
        debug_assert!(assert_adjacencies(&adjacencies));
        FatUndirectedGraph {
            vertices: adjacencies
                .into_iter()
                .map(|neighbours| VertexRec { neighbours })
                .collect(),
        }
    }
}
