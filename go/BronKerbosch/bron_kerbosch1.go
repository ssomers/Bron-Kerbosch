package BronKerbosch

func bronKerbosch1(graph *UndirectedGraph, consumer Consumer) {
	// Naive Bron-Kerbosch algorithm
	candidates := graph.connectedVertices()
	excluded := make(VertexSet, len(candidates))
	bronKerbosch1visit(graph, consumer, candidates, excluded, nil)
}

func bronKerbosch1visit(graph *UndirectedGraph, consumer Consumer,
	candidates VertexSet, excluded VertexSet, cliqueInProgress []Vertex) {
	for !candidates.IsEmpty() {
		v := candidates.PopArbitrary()
		neighbours := graph.neighbours(v)
		neighbouringCandidates := candidates.Intersection(neighbours)
		if !neighbouringCandidates.IsEmpty() {
			neighbouringExcluded := excluded.Intersection(neighbours)
			bronKerbosch1visit(graph, consumer,
				neighbouringCandidates,
				neighbouringExcluded,
				append(cliqueInProgress, v))
		} else {
			viable := len(cliqueInProgress)+1 >= consumer.MinSize
			if viable && excluded.IsDisjoint(neighbours) {
				consumer.Add(Append(cliqueInProgress, v))
			}
		}
		excluded.Add(v)
	}
}
