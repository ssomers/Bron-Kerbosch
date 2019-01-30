package bron_kerbosch

import (
	"fmt"
	"sort"
	"testing"
)

func bron_kerbosch(graph *UndirectedGraph) [][]Vertex {
	var reporter SimpleReporter
	bron_kerbosch1(graph, &reporter)
	var cliques [][]Vertex = reporter.cliques
	for _, clique := range cliques {
		sort.Slice(clique, func(l int, r int) bool {
			return clique[l] < clique[r]
		})
	}
	sort.Slice(cliques, func(l int, r int) bool {
		for i := 0; i < len(cliques[l]) && i < len(cliques[r]); i++ {
			if d := cliques[l][i] - cliques[r][i]; d != 0 {
				return d < 0
			}
		}
		panic(fmt.Sprintf("got two cliques (#%d of length %d and #%d of length %d) with the same vertices",
			l+1, len(cliques[l]),
			r+1, len(cliques[r])))
		return false
	})
	return cliques
}

func bk(t *testing.T, adjacencies []VertexSet, expected_cliques [][]Vertex) {
	graph := newUndirectedGraph(adjacencies)
	obtained_cliques := bron_kerbosch(&graph)
	if len(obtained_cliques) != len(expected_cliques) {
		t.Errorf("%d <> %d cliques", len(obtained_cliques), len(expected_cliques))
	}
	for j, o := range obtained_cliques {
		e := expected_cliques[j]
		if len(o) != len(e) {
			t.Errorf("clique #%d: %d <> %d vertices", j+1, len(o), len(e))
		} else {
			for i, l := range o {
				r := e[i]
				if l != r {
					t.Errorf("clique #%d vertex #%d/%d: %d <> %d", j+1, len(o), i+1, l, r)
				}
			}
		}
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

func TestOrder3_size_1(t *testing.T) {
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

func TestOrder3_size_2(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}},
			VertexSet{1: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1},
			[]Vertex{1, 2}})

}

func TestOrder3_size_3(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}, 2: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1, 2}})

}

func TestOrder4_size_2_isolated(t *testing.T) {
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

func TestOrder4_size_2_connected(t *testing.T) {
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

func TestOrder4_size_4_p(t *testing.T) {
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

func TestOrder4_size_4_square(t *testing.T) {
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

func TestOrder4_size_5(t *testing.T) {
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

func TestOrder4_size_6(t *testing.T) {
	bk(t,
		[]VertexSet{
			VertexSet{1: struct{}{}, 2: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 2: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}, 3: struct{}{}},
			VertexSet{0: struct{}{}, 1: struct{}{}, 2: struct{}{}}},
		[][]Vertex{
			[]Vertex{0, 1, 2, 3}})

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
