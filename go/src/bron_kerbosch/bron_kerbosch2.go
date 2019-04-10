package bron_kerbosch

func bron_kerbosch2(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with pivot of highest degree
	candidates := graph.connected_vertices()
	if candidates.IsEmpty() {
		return nil
	}
	excluded := make(VertexSet, len(candidates))
	var reporter SimpleReporter
	visit_max_degree(graph, &reporter, candidates, excluded, nil)
	return reporter.cliques
}
