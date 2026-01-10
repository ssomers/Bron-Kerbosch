package BronKerbosch

func bronKerbosch1(graph *UndirectedGraph, cliques chan<- []Vertex) {
	// Naive Bron-Kerbosch algorithm
	candidates := graph.connectedVertices()
	excluded := make(VertexSet, len(candidates))
	bronKerbosch1visit(graph, cliques, candidates, excluded, nil)
	close(cliques)
}

func bronKerbosch1visit(graph *UndirectedGraph, cliques chan<- []Vertex,
	candidates VertexSet, excluded VertexSet, clique []Vertex) {
	for !candidates.IsEmpty() {
		v := candidates.PopArbitrary()
		neighbours := graph.neighbours(v)
		neighbouringCandidates := candidates.Intersection(neighbours)
		if !neighbouringCandidates.IsEmpty() {
			neighbouringExcluded := excluded.Intersection(neighbours)
			bronKerbosch1visit(graph, cliques,
				neighbouringCandidates,
				neighbouringExcluded,
				append(clique, v))
		} else if excluded.IsDisjoint(neighbours) {
			cliques <- Append(clique, v)
		}
		excluded.Add(v)
	}
}
