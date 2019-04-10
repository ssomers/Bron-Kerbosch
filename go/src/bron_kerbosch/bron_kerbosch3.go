package bron_kerbosch

func bron_kerbosch3(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with degeneracy ordering
	var reporter SimpleReporter
	var ordering SimpleVertexVisitor
	degeneracy_ordering(graph, &ordering, -1)
	excluded := make(VertexSet, len(ordering.vertices))
	for _, v := range ordering.vertices {
		neighbours := graph.adjacencies[v]
		neighbouring_candidates := neighbours.Difference(excluded)
		if !neighbouring_candidates.IsEmpty() {
			neighbouring_excluded := neighbours.Intersection(excluded)
			visit_max_degree(
				graph, &reporter,
				neighbouring_candidates,
				neighbouring_excluded,
				[]Vertex{v})
		}
		excluded.Add(v)
	}
	return reporter.cliques
}
