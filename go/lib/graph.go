package BronKerbosch

import (
	"fmt"
)

type UndirectedGraph struct {
	adjacencies []VertexSet
}

func NewUndirectedGraph(adjacencies []VertexSet) UndirectedGraph {
	for i, adjacentToV := range adjacencies {
		v := Vertex(i)
		for w := range adjacentToV {
			if v == w {
				panic(fmt.Sprintf("%d is adjacent to itself", v))
			}
			if !adjacencies[w].Contains(v) {
				panic(fmt.Sprintf("%d is adjacent to %d but not vice versa", w, v))
			}
		}
	}
	g := UndirectedGraph{}
	g.adjacencies = adjacencies
	return g
}

func (g *UndirectedGraph) Order() int {
	return len(g.adjacencies)
}

func (g *UndirectedGraph) Size() int {
	var total int
	for _, neighbours := range g.adjacencies {
		total += neighbours.Cardinality()
	}
	if total%2 != 0 {
		panic("symmetry check should have yielded even total")
	}
	return total / 2
}

func (g *UndirectedGraph) neighbours(v Vertex) VertexSet {
	return g.adjacencies[v]
}

func (g *UndirectedGraph) degree(v Vertex) int {
	return g.adjacencies[v].Cardinality()
}

func (g *UndirectedGraph) connectedVertices() VertexSet {
	result := make(VertexSet)
	for v, neighbours := range g.adjacencies {
		if !neighbours.IsEmpty() {
			result.Add(Vertex(v))
		}
	}
	return result
}

func (g *UndirectedGraph) connectedVertexCount() int {
	var count int
	for _, neighbours := range g.adjacencies {
		if !neighbours.IsEmpty() {
			count++
		}
	}
	return count
}

func (g *UndirectedGraph) maxDegreeVertex() Vertex {
	order := g.Order()
	maxDegree := 0
	var maxVertex Vertex
	for i := 0; i < order; i++ {
		v := Vertex(i)
		degree := g.degree(v)
		if maxDegree < degree {
			maxDegree = degree
			maxVertex = v
		}
	}
	return maxVertex
}
