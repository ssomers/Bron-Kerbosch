package BronKerbosch

import (
	"BronKerboschStudy/Assert"
	"fmt"
	"testing"
)

func checkDegeneracyOrder(graph *UndirectedGraph) {
	ordered := make([]Vertex, 0)
	degeneracyVisitor(graph, func(i DegeneracyVisitItem) {
		ordered = append(ordered, i.pick)
	})
	unordered := graph.connectedVertices()
	forgotten := graph.connectedVertices() // mutable clone of unordered
	if len(ordered) > len(unordered) {
		panic(fmt.Sprintf("degeneracy ordering returns %d out of %d vertices",
			len(ordered), len(unordered)))
	}
	if len(ordered) == len(unordered) && len(unordered) > 0 {
		panic(fmt.Sprintf("degeneracy ordering returns all %d vertices", len(ordered)))
	}
	for _, v := range ordered {
		if _, ok := unordered[v]; !ok {
			panic(fmt.Sprintf("degeneracy ordering invented vertex %d", v))
		}
		if graph.degree(v) < graph.degree(ordered[0]) {
			panic(fmt.Sprintf("degeneracy ordering violates degree at vertex %d", v))
		}

		delete(forgotten, v)
		for n := range graph.neighbours(v) {
			delete(forgotten, n)
		}
	}
	if len(forgotten) > 0 {
		panic(fmt.Sprintf("degeneracy ordering forgot %d vertices", len(forgotten)))
	}
}

func proposedNeighbour(v int, proposed byte) int {
	if int(proposed) < v {
		return int(proposed)
	}
	return 1 + int(proposed)
}

func FuzzDegeneracyOrder(f *testing.F) {
	f.Add([]byte{0, 1})
	f.Fuzz(func(t *testing.T, seed []byte) {
		order := len(seed)
		for v, proposed := range seed {
			w := proposedNeighbour(v, proposed)
			Assert.IsTrue(w != v)
			if order <= w {
				order = w + 1
			}
		}
		adjacencies := make([]VertexSet, order)
		for v := range order {
			adjacencies[v] = make(VertexSet)
		}
		for v, proposed := range seed {
			w := proposedNeighbour(v, proposed)
			adjacencies[v].Add(Vertex(w))
			adjacencies[w].Add(Vertex(v))
		}
		g := NewUndirectedGraph(adjacencies)
		if g.Order() != order {
			panic("botched order")
		}
		checkDegeneracyOrder(&g)
	})
}

func bk(t *testing.T, adjacencylist [][]Vertex, expectedCliques []Clique) {
	adjacencies := make([]VertexSet, len(adjacencylist))
	for i, neighbours := range adjacencylist {
		adjacencies[i] = NewVertexSet(neighbours)
		if adjacencies[i].Cardinality() != len(neighbours) {
			panic(fmt.Sprintf("Invalid adjacencylist %v", neighbours))
		}
	}
	graph := NewUndirectedGraph(adjacencies)
	checkDegeneracyOrder(&graph)
	for funcIndex, bronKerboschFunc := range Funcs {
		cliques := make(chan Clique)
		consumer := Consumer{}
		consumer.MinSize = 2
		consumer.Accept = func(clique Clique) { cliques <- clique }

		go func() { bronKerboschFunc(&graph, consumer); close(cliques) }()

		var obtainedCliques []Clique
		for clique := range cliques {
			obtainedCliques = append(obtainedCliques, clique)
		}
		SortCliques(obtainedCliques)
		CompareCliques(obtainedCliques, expectedCliques,
			func(e string) { t.Errorf("%s: %s", FuncNames[funcIndex], e) })
	}
}

func TestOrder0(t *testing.T) {
	bk(t, [][]Vertex{}, []Clique{})
}

func TestOrder1(t *testing.T) {
	bk(t, [][]Vertex{{}}, []Clique{})
}

func TestOrder2isolated(t *testing.T) {
	bk(t,
		[][]Vertex{
			{},
			{}},
		[]Clique{})
}

func TestOrder2connected(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1},
			{0}},
		[]Clique{
			{0, 1}})
}

func TestOrder3Size1left(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1},
			{0},
			{}},
		[]Clique{
			{0, 1}})
}

func TestOrder3Size1long(t *testing.T) {
	bk(t,
		[][]Vertex{
			{2},
			{},
			{0}},
		[]Clique{
			{0, 2}})
}

func TestOrder3Size1right(t *testing.T) {
	bk(t,
		[][]Vertex{
			{},
			{2},
			{1}},
		[]Clique{
			{1, 2}})
}

func TestOrder3Size2(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1},
			{0, 2},
			{1}},
		[]Clique{
			{0, 1},
			{1, 2}})
}

func TestOrder3Size3(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1, 2},
			{0, 2},
			{0, 1}},
		[]Clique{
			{0, 1, 2}})
}

func TestOrder4Size2(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1},
			{0},
			{3},
			{2}},
		[]Clique{
			{0, 1},
			{2, 3}})
}

func TestOrder4Size3bus(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1},
			{0, 2},
			{1, 3},
			{2}},
		[]Clique{
			{0, 1},
			{1, 2},
			{2, 3}})
}

func TestOrder4Size3star(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1, 2, 3},
			{0},
			{0},
			{0}},
		[]Clique{
			{0, 1},
			{0, 2},
			{0, 3}})
}

func TestOrder4Size4p(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1},
			{0, 2, 3},
			{1, 3},
			{1, 2}},
		[]Clique{
			{0, 1},
			{1, 2, 3}})
}

func TestOrder4Size4square(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1, 3},
			{0, 2},
			{1, 3},
			{0, 2}},
		[]Clique{
			{0, 1},
			{0, 3},
			{1, 2},
			{2, 3},
		})
}

func TestOrder4Size5(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1, 2, 3},
			{0, 2},
			{0, 1, 3},
			{0, 2}},
		[]Clique{
			{0, 1, 2},
			{0, 2, 3}})
}

func TestOrder4Size6(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1, 2, 3},
			{0, 2, 3},
			{0, 1, 3},
			{0, 1, 2}},
		[]Clique{
			{0, 1, 2, 3}})
}

func TestOrder5Size9penultimate(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1, 2, 3, 4},
			{0, 2, 3, 4},
			{0, 1, 3, 4},
			{0, 1, 2},
			{0, 1, 2}},
		[]Clique{
			{0, 1, 2, 3},
			{0, 1, 2, 4}})
}

func TestSample(t *testing.T) {
	bk(t,
		[][]Vertex{
			{},
			{2, 3, 4},
			{1, 3, 4, 5},
			{1, 2, 4, 5},
			{1, 2, 3},
			{2, 3, 6, 7},
			{5, 7},
			{5, 6}},
		[]Clique{
			{1, 2, 3, 4},
			{2, 3, 5},
			{5, 6, 7},
		})
}

func TestBigger(t *testing.T) {
	bk(t,
		[][]Vertex{
			{1, 2, 3, 4, 6, 7},
			{0, 3, 6, 7, 8, 9},
			{0, 3, 5, 7, 8, 9},
			{0, 1, 2, 4, 9},
			{0, 3, 6, 7, 9},
			{2, 6},
			{0, 1, 4, 5, 9},
			{0, 1, 2, 4, 9},
			{1, 2},
			{1, 2, 3, 4, 6, 7}},
		[]Clique{
			{0, 1, 3},
			{0, 1, 6},
			{0, 1, 7},
			{0, 2, 3},
			{0, 2, 7},
			{0, 3, 4},
			{0, 4, 6},
			{0, 4, 7},
			{1, 3, 9},
			{1, 6, 9},
			{1, 7, 9},
			{1, 8},
			{2, 3, 9},
			{2, 5},
			{2, 7, 9},
			{2, 8},
			{3, 4, 9},
			{4, 6, 9},
			{4, 7, 9},
			{5, 6},
		})
}
