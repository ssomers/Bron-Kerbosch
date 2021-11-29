package BronKerbosch

func bronKerbosch1(graph *UndirectedGraph, reporter Reporter) {
	// Naive Bron-Kerbosch algorithm
	candidates := graph.connectedVertices()
	if candidates.IsEmpty() {
		return
	}
	excluded := make(VertexSet, len(candidates))
	bronKerbosch1visit(graph, reporter, candidates, excluded, nil)
}

func bronKerbosch1visit(graph *UndirectedGraph, reporter Reporter,
	candidates VertexSet, excluded VertexSet, clique []Vertex) {
	for !candidates.IsEmpty() {
		v := candidates.PopArbitrary()
		neighbours := graph.neighbours(v)
		neighbouringCandidates := candidates.Intersection(neighbours)
		if !neighbouringCandidates.IsEmpty() {
			neighbouringExcluded := excluded.Intersection(neighbours)
			bronKerbosch1visit(graph, reporter,
				neighbouringCandidates,
				neighbouringExcluded,
				append(clique, v))
		} else if excluded.IsDisjoint(neighbours) {
			reporter.Record(append(clique, v))
		}
		excluded.Add(v)
	}
}
