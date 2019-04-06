package bron_kerbosch

import (
	"fmt"
	"testing"
)

func check_degeneracy_order(graph *UndirectedGraph) {
	ordered := degeneracy_ordering(graph)
	unordered := graph.connected_vertices()
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

func bk(t *testing.T, adjacencylist [][]Vertex, expected_cliques [][]Vertex) {
	adjacencies := make([]VertexSet, len(adjacencylist))
	for i, neighbours := range adjacencylist {
		adjacencies[i] = NewVertexSet(neighbours)
		if adjacencies[i].Cardinality() != len(neighbours) {
			panic(fmt.Sprintf("Invalid adjacencylist %v", neighbours))
		}
	}
	graph := newUndirectedGraph(adjacencies)
	check_degeneracy_order(&graph)
	for func_index, bron_kerbosch_func := range FUNCS {
		var reporter SimpleReporter
		bron_kerbosch_func(&graph, &reporter)
		obtained_cliques := reporter.cliques
		sort_cliques(obtained_cliques)
		compare_cliques(obtained_cliques, expected_cliques,
			func(e string) { t.Errorf("%s: %s", FUNC_NAMES[func_index], e) })
	}
}

func TestOrder0(t *testing.T) {
	bk(t, [][]Vertex{}, [][]Vertex{})
}

func TestOrder1(t *testing.T) {
	bk(t, [][]Vertex{[]Vertex{}}, [][]Vertex{})
}

func TestOrder2_isolated(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{},
			[]Vertex{}},
		[][]Vertex{})
}

func TestOrder2_connected(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1},
			[]Vertex{0}},
		[][]Vertex{
			[]Vertex{0, 1}})
}

func TestOrder3_Size1(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1},
			[]Vertex{0},
			[]Vertex{}},
		[][]Vertex{
			[]Vertex{0, 1}})
	bk(t,
		[][]Vertex{
			[]Vertex{},
			[]Vertex{2},
			[]Vertex{1}},
		[][]Vertex{
			[]Vertex{1, 2}})
}

func TestOrder3_Size2(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1},
			[]Vertex{0, 2},
			[]Vertex{1}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{1, 2}})
}

func TestOrder3_Size3(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 2},
			[]Vertex{0, 2},
			[]Vertex{0, 1}},
		[][]Vertex{
			[]Vertex{0, 1, 2}})
}

func TestOrder4_Size2_isolated(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 2},
			[]Vertex{0},
			[]Vertex{0},
			[]Vertex{}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{0, 2}})
}

func TestOrder4_Size2_connected(t *testing.T) {
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

func TestOrder4_Size4_p(t *testing.T) {
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

func TestOrder4_Size4_square(t *testing.T) {
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

func TestOrder4_Size5(t *testing.T) {
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

func TestOrder4_Size6(t *testing.T) {
	bk(t,
		[][]Vertex{
			[]Vertex{1, 2, 3},
			[]Vertex{0, 2, 3},
			[]Vertex{0, 1, 3},
			[]Vertex{0, 1, 2}},
		[][]Vertex{
			[]Vertex{0, 1, 2, 3}})
}

func TestOrder5_Size9_penultimate(t *testing.T) {
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
