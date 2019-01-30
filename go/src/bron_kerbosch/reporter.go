package bron_kerbosch

type Reporter interface {
	Record(clique []Vertex)
}

type SimpleReporter struct {
	cliques [][]Vertex
}

func (r *SimpleReporter) Record(clique []Vertex) {
	cc := make([]Vertex, len(clique))
	copy(cc, clique)
	r.cliques = append(r.cliques, cc)
}
