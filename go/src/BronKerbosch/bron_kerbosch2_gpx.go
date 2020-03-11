package BronKerbosch

func bronKerbosch2gpx(graph *UndirectedGraph, reporter Reporter) {
	// Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
	// chosen from both candidates and excluded vertices (IK_GPX).
	candidates := graph.connectedVertices()
	if candidates.IsEmpty() {
		return
	}
	excluded := make(VertexSet, len(candidates))
	visit(graph, reporter, MaxDegree, MaxDegreeLocalX, candidates, excluded, nil)
}
