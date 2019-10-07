package BronKerbosch

func bronKerbosch1(graph *UndirectedGraph) [][]Vertex {
	// Naive Bron-Kerbosch algorithm
	candidates := graph.connectedVertices()
	if candidates.IsEmpty() {
		return nil
	}
	excluded := make(VertexSet, len(candidates))
	var reporter SimpleReporter
	bronKerbosch1visit(graph, &reporter, candidates, excluded, nil)
	return reporter.cliques
}

func bronKerbosch1visit(graph *UndirectedGraph, reporter Reporter,
	candidates VertexSet, excluded VertexSet, clique []Vertex) {
	for {
		v := candidates.PopArbitrary()
		neighbours := graph.adjacencies[v]
		neighbouringCandidates := candidates.Intersection(neighbours)
		if !neighbouringCandidates.IsEmpty() {
			neighbouringExcluded := excluded.Intersection(neighbours)
			bronKerbosch1visit(graph, reporter,
				neighbouringCandidates,
				neighbouringExcluded,
				append(clique, v))
		} else {
			if excluded.IsDisjoint(neighbours) {
				reporter.Record(append(clique, v))
			}
			if candidates.IsEmpty() {
				break
			}
		}
		excluded.Add(v)
	}
}
