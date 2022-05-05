package BronKerbosch

func bronKerbosch2aGP(graph *UndirectedGraph, reporter Reporter) {
	// Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
	// chosen from candidates only (IK_GP).
	candidates := graph.connectedVertices()
	if candidates.IsEmpty() {
		return
	}
	excluded := make(VertexSet, len(candidates))
	visit(graph, reporter, MaxDegreeLocal, candidates, excluded, nil)
}
