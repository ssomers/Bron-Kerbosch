package BronKerbosch

func bronKerbosch3gp(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
	// choosing a pivot from candidates only (IK_GP).
	var reporter SimpleReporter
	var ordering SimpleVertexVisitor
	degeneracyOrdering(graph, &ordering, -1)
	excluded := make(VertexSet, len(ordering.vertices))
	for _, v := range ordering.vertices {
		neighbours := graph.adjacencies[v]
		neighbouringCandidates := neighbours.Difference(excluded)
		if !neighbouringCandidates.IsEmpty() {
			neighbouringExcluded := neighbours.Intersection(excluded)
			visit(
				graph, &reporter,
				MaxDegree, MaxDegreeLocal,
				neighbouringCandidates,
				neighbouringExcluded,
				[]Vertex{v})
		}
		excluded.Add(v)
	}
	return reporter.cliques
}