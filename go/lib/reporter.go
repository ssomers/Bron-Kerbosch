package BronKerbosch

type Reporter interface {
	Record(clique []Vertex)
}

type SimpleReporter struct {
	Cliques [][]Vertex
}

func (r *SimpleReporter) Record(clique []Vertex) {
	cc := make([]Vertex, len(clique))
	copy(cc, clique)
	r.Cliques = append(r.Cliques, cc)
}

type CountingReporter struct {
	Cliques int
}

func (r *CountingReporter) Record(clique []Vertex) {
	r.Cliques += 1
}

type ChannelReporter struct {
	cliques chan<- []Vertex
}

func (r *ChannelReporter) Record(clique []Vertex) {
	cc := make([]Vertex, len(clique))
	copy(cc, clique)
	r.cliques <- cc
}

func gatherCliques(cliques <-chan []Vertex, finalReporter Reporter) {
	for clique := range cliques {
		finalReporter.Record(clique)
	}
}
