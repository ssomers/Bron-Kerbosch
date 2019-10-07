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

type ChannelReporter struct {
	cliques chan<- []Vertex
}

func (r *ChannelReporter) Record(clique []Vertex) {
	cc := make([]Vertex, len(clique))
	copy(cc, clique)
	r.cliques <- cc
}

func gather_cliques(cliques <-chan []Vertex) [][]Vertex {
	var result [][]Vertex
	for clique := range cliques {
		result = append(result, clique)
	}
	return result
}
