package bron_kerbosch

func bron_kerbosch1(graph *UndirectedGraph, reporter Reporter) {
	// Naive Bron-Kerbosch algorithm
	candidates := graph.connected_nodes()
	if !candidates.IsEmpty() {
		var excluded VertexSet
		bron_kerbosch1_visit(
			graph,
			reporter,
			&candidates,
			&excluded,
			[]Vertex{})
	}
}

func bron_kerbosch1_visit(graph *UndirectedGraph, reporter Reporter, candidates *VertexSet,
	excluded *VertexSet, clique []Vertex) {
	if candidates.IsEmpty() && excluded.IsEmpty() {
		reporter.Record(clique)
	}

	for !candidates.IsEmpty() {
		v := candidates.PopArbitrary()
		neighbours := &graph.adjacencies[v]
		neighbouring_candidates := candidates.Intersection(neighbours)
		neighbouring_excluded := excluded.Intersection(neighbours)
		bron_kerbosch1_visit(graph, reporter,
			&neighbouring_candidates,
			&neighbouring_excluded,
			append(clique, v))
		excluded.Add(v)
	}
}
