package BronKerbosch

func bronKerbosch2gpx(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
	// chosen from both candidates and excluded vertices (IK_GPX).
	candidates := graph.connectedVertices()
	if candidates.IsEmpty() {
		return nil
	}
	excluded := make(VertexSet, len(candidates))
	var reporter SimpleReporter
	visit(graph, &reporter, MaxDegree, MaxDegreeLocalX, candidates, excluded, nil)
	return reporter.cliques
}
