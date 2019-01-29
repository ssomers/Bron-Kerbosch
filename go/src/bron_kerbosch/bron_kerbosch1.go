package bron_kerbosch

func bron_kerbosch1(graph *UndirectedGraph, reporter Reporter) {
	// Naive Bron-Kerbosch algorithm
	candidates := graph.connected_nodes()
	if len(candidates) != 0 {
		excluded := make(VertexSet)
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
	if len(*candidates) == 0 && len(*excluded) == 0 {
		reporter.Record(clique)
	}

	for len(*candidates) != 0 {
		v := pop_arbitrary(candidates)
		neighbours := &graph.adjacencies[v]
		neighbouring_candidates := intersection(candidates, neighbours)
		neighbouring_excluded := intersection(excluded, neighbours)
		bron_kerbosch1_visit(graph, reporter,
			&neighbouring_candidates,
			&neighbouring_excluded,
			append(clique, v))
		(*excluded)[v] = struct{}{}
	}
}
