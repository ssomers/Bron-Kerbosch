package BronKerbosch

func bronKerbosch3gp(graph *UndirectedGraph, cliques chan<- []Vertex) {
	// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
	// choosing a pivot from candidates only (IK_GP).
	var ordering SimpleVertexVisitor
	degeneracyOrdering(graph, &ordering)
	// In this initial iteration, we don't need to represent the set of candidates
	// because all neighbours are candidates until excluded.
	excluded := make(VertexSet, len(ordering.vertices))
	for _, v := range ordering.vertices {
		neighbours := graph.neighbours(v)
		neighbouringExcluded := neighbours.Intersection(excluded)
		if len(neighbouringExcluded) < len(neighbours) {
			neighbouringCandidates := neighbours.Difference(neighbouringExcluded)
			visit(
				graph, cliques,
				MaxDegreeLocal,
				neighbouringCandidates,
				neighbouringExcluded,
				[]Vertex{v})
		}
		excluded.Add(v)
	}
	close(cliques)
}
