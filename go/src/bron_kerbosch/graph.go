package bron_kerbosch

import (
	"fmt"
)

type UndirectedGraph struct {
	adjacencies []VertexSet
}

func newUndirectedGraph(adjacencies []VertexSet) UndirectedGraph {
	for i, adjacent_to_v := range adjacencies {
		v := Vertex(i)
		for w, _ := range adjacent_to_v {
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
	for _, a := range g.adjacencies {
		total += a.Cardinality()
	}
	if total%2 != 0 {
		panic("symmetry check should have yielded even total")
	}
	return total / 2
}

func (g *UndirectedGraph) degree(v Vertex) int {
	return g.adjacencies[v].Cardinality()
}

func (g *UndirectedGraph) connected_vertices() VertexSet {
	order := g.order()
	result := make(VertexSet)
	for v := 0; v < order; v++ {
		if !g.adjacencies[v].IsEmpty() {
			result.Add(Vertex(v))
		}
	}
	return result
}

func random_undirected_graph(order int, size int) UndirectedGraph {
	fully_meshed_size := order * (order - 1) / 2
	if size > fully_meshed_size {
		panic(
			fmt.Sprintf("%d nodes accommodate at most %d edges", order, fully_meshed_size))
	}
	unsaturated_vertices := make([]Vertex, order)
	adjacency_sets := make([]VertexSet, order)
	for v := 0; v < order; v++ {
		unsaturated_vertices[v] = Vertex(v)
		adjacency_sets[v] = make(VertexSet)
	}
	adjacency_complements := make([]VertexSet, order)
	for i := 0; i < size; i++ {
		v := random_choice(&unsaturated_vertices)
		if adjacency_sets[v].Cardinality() >= order-1 {
			panic("too many adjacencies")
		}
		var w Vertex
		if !adjacency_complements[v].IsEmpty() {
			w = random_sample(&adjacency_complements[v])
		} else {
			w = v
			for w == v || adjacency_sets[v].Contains(w) {
				w = random_choice(&unsaturated_vertices)
			}
		}
		if v == w {
			panic("defecation has hit oscillation")
		}
		if adjacency_sets[v].Contains(w) {
			panic("defecation has hit oscillation")
		}
		if adjacency_sets[w].Contains(v) {
			panic("defecation has hit oscillation")
		}
		pairs := [...]struct {
			x Vertex
			y Vertex
		}{{v, w}, {w, v}}
		for _, pair := range pairs {
			x := pair.x
			y := pair.y
			adjacency_sets[x].Add(y)
			neighbours := adjacency_sets[x].Cardinality()
			if neighbours == order-1 {
				unsaturated_vertices = array_remove(unsaturated_vertices, x)
			} else if neighbours == order/2 {
				// start using adjacency complement
				if !adjacency_complements[x].IsEmpty() {
					panic("unexpected adjacency_complements")
				}
				adjacency_complements[x] = make(VertexSet)
				for _, v := range unsaturated_vertices {
					if v != x {
						if !adjacency_sets[x].Contains(v) {
							adjacency_complements[x].Add(v)
						}
					}
				}
			} else if neighbours > order/2 {
				adjacency_complements[x].Remove(y)
			}
		}
	}
	var g UndirectedGraph
	g.adjacencies = adjacency_sets
	if g.order() != order {
		panic("botched order")
	}
	if g.size() != size {
		panic("botched size")
	}
	return g
}
