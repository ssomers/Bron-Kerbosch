package BronKerbosch

func bronKerbosch2g(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with pivot of highest degree within the whole graph
	candidates := graph.connectedVertices()
	if candidates.IsEmpty() {
		return nil
	}
	excluded := make(VertexSet, len(candidates))
	var reporter SimpleReporter
	visit(graph, &reporter, MaxDegree, MaxDegree, candidates, excluded, nil)
	return reporter.cliques
}
