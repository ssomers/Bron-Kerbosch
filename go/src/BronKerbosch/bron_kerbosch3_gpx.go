package BronKerbosch

func bronKerbosch3gpx(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
	// choosing a pivot from both candidates and excluded vertices (IK_GPX).
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
				MaxDegree, MaxDegreeLocalX,
				neighbouringCandidates,
				neighbouringExcluded,
				[]Vertex{v})
		}
		excluded.Add(v)
	}
	return reporter.cliques
}
