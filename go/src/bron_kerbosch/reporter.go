package bron_kerbosch

type Reporter interface {
	Record(clique []Vertex)
}

type SimpleReporter struct {
	cliques [][]Vertex
}

func (r *SimpleReporter) Record(clique []Vertex) {
	r.cliques = append(r.cliques, clique)
}
