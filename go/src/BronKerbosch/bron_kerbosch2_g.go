package BronKerbosch

func bronKerbosch2g(graph *UndirectedGraph, reporter Reporter) {
	// Bron-Kerbosch algorithm with pivot of highest degree within the whole graph
	candidates := graph.connectedVertices()
	if candidates.IsEmpty() {
		return
	}
	excluded := make(VertexSet, len(candidates))
	visit(graph, reporter, MaxDegree, MaxDegree, candidates, excluded, nil)
}
