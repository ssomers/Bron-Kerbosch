package bron_kerbosch

func bron_kerbosch2_g(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with pivot of highest degree within the whole graph
	candidates := graph.connected_vertices()
	if candidates.IsEmpty() {
		return nil
	}
	excluded := make(VertexSet, len(candidates))
	var reporter SimpleReporter
	visit(graph, &reporter, MaxDegree, MaxDegree, candidates, excluded, nil)
	return reporter.cliques
}