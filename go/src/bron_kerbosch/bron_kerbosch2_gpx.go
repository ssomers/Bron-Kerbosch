package bron_kerbosch

func bron_kerbosch2_gpx(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
	// chosen from both candidates and excluded vertices (IK_GPX).
	candidates := graph.connected_vertices()
	if candidates.IsEmpty() {
		return nil
	}
	excluded := make(VertexSet, len(candidates))
	var reporter SimpleReporter
	visit(graph, &reporter, MaxDegree, MaxDegreeLocalX, candidates, excluded, nil)
	return reporter.cliques
}