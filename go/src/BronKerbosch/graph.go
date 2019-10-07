package BronKerbosch

import (
	"fmt"
)

type UndirectedGraph struct {
	adjacencies []VertexSet
}

func newUndirectedGraph(adjacencies []VertexSet) UndirectedGraph {
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

func (g *UndirectedGraph) order() int {
	return len(g.adjacencies)
}

func (g *UndirectedGraph) size() int {
	var total int
	for _, neighbours := range g.adjacencies {
		total += neighbours.Cardinality()
	}
	if total%2 != 0 {
		panic("symmetry check should have yielded even total")
	}
	return total / 2
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

func randomUndirectedGraph(order int, size int) UndirectedGraph {
	fullyMeshedSize := order * (order - 1) / 2
	if size > fullyMeshedSize {
		panic(
			fmt.Sprintf("%d nodes accommodate at most %d edges", order, fullyMeshedSize))
	}
	unsaturatedVertices := make([]Vertex, order)
	adjacencySets := make([]VertexSet, order)
	for v := 0; v < order; v++ {
		unsaturatedVertices[v] = Vertex(v)
		adjacencySets[v] = make(VertexSet)
	}
	adjacencyComplements := make([]VertexSet, order)
	for i := 0; i < size; i++ {
		v := randomChoice(&unsaturatedVertices)
		if adjacencySets[v].Cardinality() >= order-1 {
			panic("too many adjacencies")
		}
		var w Vertex
		if !adjacencyComplements[v].IsEmpty() {
			w = randomSample(&adjacencyComplements[v])
		} else {
			w = v
			for w == v || adjacencySets[v].Contains(w) {
				w = randomChoice(&unsaturatedVertices)
			}
		}
		if v == w {
			panic("defecation has hit oscillation")
		}
		if adjacencySets[v].Contains(w) {
			panic("defecation has hit oscillation")
		}
		if adjacencySets[w].Contains(v) {
			panic("defecation has hit oscillation")
		}
		pairs := [...]struct {
			x Vertex
			y Vertex
		}{{v, w}, {w, v}}
		for _, pair := range pairs {
			x := pair.x
			y := pair.y
			adjacencySets[x].Add(y)
			neighbours := adjacencySets[x].Cardinality()
			if neighbours == order-1 {
				unsaturatedVertices = removeFromArray(unsaturatedVertices, x)
			} else if neighbours == order/2 {
				// start using adjacency complement
				if !adjacencyComplements[x].IsEmpty() {
					panic("unexpected adjacencyComplements")
				}
				adjacencyComplements[x] = make(VertexSet)
				for _, v := range unsaturatedVertices {
					if v != x {
						if !adjacencySets[x].Contains(v) {
							adjacencyComplements[x].Add(v)
						}
					}
				}
			} else if neighbours > order/2 {
				adjacencyComplements[x].Remove(y)
			}
		}
	}
	var g UndirectedGraph
	g.adjacencies = adjacencySets
	if g.order() != order {
		panic("botched order")
	}
	if g.size() != size {
		panic("botched size")
	}
	return g
}
