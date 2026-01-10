package BronKerbosch

func bronKerbosch2aGP(graph *UndirectedGraph, cliques chan<- []Vertex) {
	// Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
	// chosen from candidates only (IK_GP).
	candidates := graph.connectedVertices()
	excluded := make(VertexSet, len(candidates))
	visit(graph, cliques, MaxDegreeLocal, candidates, excluded, nil)
	close(cliques)
}
