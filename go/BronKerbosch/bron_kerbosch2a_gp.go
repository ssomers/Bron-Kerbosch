package BronKerbosch

func bronKerbosch2aGP(graph *UndirectedGraph, consumer Consumer) {
	// Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
	// chosen from candidates only (IK_GP).
	candidates := graph.connectedVertices()
	excluded := make(VertexSet, len(candidates))
	visit(graph, consumer, MaxDegreeLocal, candidates, excluded, nil)
	consumer.close()
}
