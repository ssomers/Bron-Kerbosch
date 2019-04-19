package bron_kerbosch

func bron_kerbosch3_gp(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with degeneracy ordering,
	// recursing with pivot of highest degree (IK_GP)
	var reporter SimpleReporter
	var ordering SimpleVertexVisitor
	degeneracy_ordering(graph, &ordering, -1)
	excluded := make(VertexSet, len(ordering.vertices))
	for _, v := range ordering.vertices {
		neighbours := graph.adjacencies[v]
		neighbouring_candidates := neighbours.Difference(excluded)
		if !neighbouring_candidates.IsEmpty() {
			neighbouring_excluded := neighbours.Intersection(excluded)
			visit(
				graph, &reporter,
				MaxDegree, MaxDegree,
				neighbouring_candidates,
				neighbouring_excluded,
				[]Vertex{v})
		}
		excluded.Add(v)
	}
	return reporter.cliques
}
