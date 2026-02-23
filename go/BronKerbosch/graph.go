package BronKerbosch

import (
	"fmt"
)

// Designed and to be used entirely immutably.
type UndirectedGraph struct {
	adjacencies          []VertexSet
	size                 int
	max_degree           int
	connectedVertexCount int
}

func NewUndirectedGraph(adjacencies []VertexSet) UndirectedGraph {
	connectedVertexCount := 0
	max_degree := 0
	total_degree := 0
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
		degree := len(adjacentToV)
		if degree > 0 {
			total_degree += degree
			if max_degree < degree {
				max_degree = degree
			}
			connectedVertexCount += 1
		}
	}
	if total_degree%2 != 0 {
		panic("symmetry check should have enforced even total")
	}
	g := UndirectedGraph{}
	g.adjacencies = adjacencies
	g.size = total_degree / 2
	g.max_degree = max_degree
	g.connectedVertexCount = connectedVertexCount
	return g
}

func (g *UndirectedGraph) Order() int {
	return len(g.adjacencies)
}

func (g *UndirectedGraph) Size() int {
	return g.size
}

func (g *UndirectedGraph) MaxDegree() int {
	return g.max_degree
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

func (g *UndirectedGraph) maxDegreeVertex() Vertex {
	for v, neighbours := range g.adjacencies {
		if len(neighbours) == g.max_degree {
			return Vertex(v)
		}
	}
	panic("attempt to find max-degree vertex in empty graph")
}
