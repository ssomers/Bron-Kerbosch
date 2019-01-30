package bron_kerbosch

import (
	"testing"
)

func bk(t *testing.T, adjacencies []VertexSet, expected_cliques [][]Vertex) {
	graph := newUndirectedGraph(adjacencies)
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
	bk(t, []VertexSet{}, [][]Vertex{})
}

func TestOrder1(t *testing.T) {
	bk(t, []VertexSet{VertexSet{}}, [][]Vertex{})
}

func TestOrder2_isolated(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{},
			VertexSet{}},
		[][]Vertex{})
}

func TestOrder2_connected(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}},
			VertexSet{0: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1}})
}

func TestOrder3_Size1(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}},
			VertexSet{0: struct{}{}},
			VertexSet{}},
		[][]Vertex{
			[]Vertex{0, 1}})
	bk(t,
		[]VertexSet{
			VertexSet{},
			VertexSet{2: struct{}{}},
			VertexSet{1: struct{}{}}},
		[][]Vertex{
			[]Vertex{1, 2}})
}

func TestOrder3_Size2(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}},
			VertexSet{1: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{1, 2}})
}

func TestOrder3_Size3(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}, 2: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1, 2}})
}

func TestOrder4_Size2_isolated(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}, 2: struct{}{}},
			VertexSet{0: struct{}{}},
			VertexSet{0: struct{}{}},
			VertexSet{}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{0, 2}})
}

func TestOrder4_Size2_connected(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}},
			VertexSet{0: struct{}{}},
			VertexSet{3: struct{}{}},
			VertexSet{2: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{2, 3}})
}

func TestOrder4_Size4_p(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}, 3: struct{}{}},
			VertexSet{1: struct{}{}, 3: struct{}{}},
			VertexSet{1: struct{}{}, 2: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{1, 2, 3}})
}

func TestOrder4_Size4_square(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}},
			VertexSet{1: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{0, 3},
			[]Vertex{1, 2},
			[]Vertex{2, 3},
		})
}

func TestOrder4_Size5(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}, 2: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1, 2},
			[]Vertex{0, 2, 3}})
}

func TestOrder4_Size6(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}, 2: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}, 2: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1, 2, 3}})
}

func TestOrder5_Size9_penultimate(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}, 2: struct{}{}, 3: struct{}{}, 4: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}, 3: struct{}{}, 4: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}, 3: struct{}{}, 4: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}, 2: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}, 2: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1, 2, 3},
			[]Vertex{0, 1, 2, 4}})
}

func TestSample(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{},
			VertexSet{2: struct{}{}, 3: struct{}{}, 4: struct{}{}},
			VertexSet{1: struct{}{}, 3: struct{}{}, 4: struct{}{}, 5: struct{}{}},
			VertexSet{1: struct{}{}, 2: struct{}{}, 4: struct{}{}, 5: struct{}{}},
			VertexSet{1: struct{}{}, 2: struct{}{}, 3: struct{}{}},
			VertexSet{2: struct{}{}, 3: struct{}{}, 6: struct{}{}, 7: struct{}{}},
			VertexSet{5: struct{}{}, 7: struct{}{}},
			VertexSet{5: struct{}{}, 6: struct{}{}}},
		[][]Vertex{
			[]Vertex{1, 2, 3, 4},
			[]Vertex{2, 3, 5},
			[]Vertex{5, 6, 7},
		})
}
