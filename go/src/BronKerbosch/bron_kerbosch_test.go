package BronKerbosch

import (
	"fmt"
	"testing"
)

func checkDegeneracyOrder(graph *UndirectedGraph) {
	var ordering SimpleVertexVisitor
	degeneracyOrdering(graph, &ordering, 0)
	ordered := ordering.vertices
	unordered := graph.connectedVertices()
	for _, v := range ordered {
		if _, ok := unordered[v]; !ok {
			panic(fmt.Sprintf("degeneracy ordering %v invented vertex %d", ordered, v))
		}
		delete(unordered, v)

		if graph.degree(v) < graph.degree(ordered[0]) {
			panic(fmt.Sprintf("degeneracy ordering %v violates degree at vertex %d", ordered, v))
		}
	}
	if len(unordered) != 0 {
		panic(fmt.Sprintf("degeneracy ordering %v forgot %d vertices", ordered, len(unordered)))
	}
}

func bk(t *testing.T, adjacencylist [][]Vertex, expectedCliques [][]Vertex) {
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
		obtainedCliques := bronKerboschFunc(&graph)
		SortCliques(obtainedCliques)
		CompareCliques(obtainedCliques, expectedCliques,
			func(e string) { t.Errorf("%s: %s", FuncNames[funcIndex], e) })
	}
}

func TestOrder0(t *testing.T) {
	bk(t, [][]Vertex{}, [][]Vertex{})
}

func TestOrder1(t *testing.T) {
	bk(t, [][]Vertex{[]Vertex{}}, [][]Vertex{})
}

func TestOrder2isolated(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{},
			[]Vertex{}},
		[][]Vertex{})
}

func TestOrder2connected(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1},
			[]Vertex{0}},
		[][]Vertex{
			[]Vertex{0, 1}})
}

func TestOrder3Size1left(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1},
			[]Vertex{0},
			[]Vertex{}},
		[][]Vertex{
			[]Vertex{0, 1}})
}

func TestOrder3Size1long(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{2},
			[]Vertex{},
			[]Vertex{0}},
		[][]Vertex{
			[]Vertex{0, 2}})
}

func TestOrder3Size1right(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{},
			[]Vertex{2},
			[]Vertex{1}},
		[][]Vertex{
			[]Vertex{1, 2}})
}

func TestOrder3Size2(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1},
			[]Vertex{0, 2},
			[]Vertex{1}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{1, 2}})
}

func TestOrder3Size3(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 2},
			[]Vertex{0, 2},
			[]Vertex{0, 1}},
		[][]Vertex{
			[]Vertex{0, 1, 2}})
}

func TestOrder4Size2(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1},
			[]Vertex{0},
			[]Vertex{3},
			[]Vertex{2}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{2, 3}})
}

func TestOrder4Size3bus(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1},
			[]Vertex{0, 2},
			[]Vertex{1, 3},
			[]Vertex{2}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{1, 2},
			[]Vertex{2, 3}})
}

func TestOrder4Size3star(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 2, 3},
			[]Vertex{0},
			[]Vertex{0},
			[]Vertex{0}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{0, 2},
			[]Vertex{0, 3}})
}

func TestOrder4Size4p(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1},
			[]Vertex{0, 2, 3},
			[]Vertex{1, 3},
			[]Vertex{1, 2}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{1, 2, 3}})
}

func TestOrder4Size4square(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 3},
			[]Vertex{0, 2},
			[]Vertex{1, 3},
			[]Vertex{0, 2}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{0, 3},
			[]Vertex{1, 2},
			[]Vertex{2, 3},
		})
}

func TestOrder4Size5(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 2, 3},
			[]Vertex{0, 2},
			[]Vertex{0, 1, 3},
			[]Vertex{0, 2}},
		[][]Vertex{
			[]Vertex{0, 1, 2},
			[]Vertex{0, 2, 3}})
}

func TestOrder4Size6(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 2, 3},
			[]Vertex{0, 2, 3},
			[]Vertex{0, 1, 3},
			[]Vertex{0, 1, 2}},
		[][]Vertex{
			[]Vertex{0, 1, 2, 3}})
}

func TestOrder5Size9penultimate(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 2, 3, 4},
			[]Vertex{0, 2, 3, 4},
			[]Vertex{0, 1, 3, 4},
			[]Vertex{0, 1, 2},
			[]Vertex{0, 1, 2}},
		[][]Vertex{
			[]Vertex{0, 1, 2, 3},
			[]Vertex{0, 1, 2, 4}})
}

func TestSample(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{},
			[]Vertex{2, 3, 4},
			[]Vertex{1, 3, 4, 5},
			[]Vertex{1, 2, 4, 5},
			[]Vertex{1, 2, 3},
			[]Vertex{2, 3, 6, 7},
			[]Vertex{5, 7},
			[]Vertex{5, 6}},
		[][]Vertex{
			[]Vertex{1, 2, 3, 4},
			[]Vertex{2, 3, 5},
			[]Vertex{5, 6, 7},
		})
}

func TestBigger(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 2, 3, 4, 6, 7},
			[]Vertex{0, 3, 6, 7, 8, 9},
			[]Vertex{0, 3, 5, 7, 8, 9},
			[]Vertex{0, 1, 2, 4, 9},
			[]Vertex{0, 3, 6, 7, 9},
			[]Vertex{2, 6},
			[]Vertex{0, 1, 4, 5, 9},
			[]Vertex{0, 1, 2, 4, 9},
			[]Vertex{1, 2},
			[]Vertex{1, 2, 3, 4, 6, 7}},
		[][]Vertex{
			[]Vertex{0, 1, 3},
			[]Vertex{0, 1, 6},
			[]Vertex{0, 1, 7},
			[]Vertex{0, 2, 3},
			[]Vertex{0, 2, 7},
			[]Vertex{0, 3, 4},
			[]Vertex{0, 4, 6},
			[]Vertex{0, 4, 7},
			[]Vertex{1, 3, 9},
			[]Vertex{1, 6, 9},
			[]Vertex{1, 7, 9},
			[]Vertex{1, 8},
			[]Vertex{2, 3, 9},
			[]Vertex{2, 5},
			[]Vertex{2, 7, 9},
			[]Vertex{2, 8},
			[]Vertex{3, 4, 9},
			[]Vertex{4, 6, 9},
			[]Vertex{4, 7, 9},
			[]Vertex{5, 6},
		})
}
