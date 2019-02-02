package bron_kerbosch

func bron_kerbosch2(graph *UndirectedGraph, reporter Reporter) {
	// Bron-Kerbosch algorithm with pivot picked arbitrarily
	candidates := graph.connected_nodes()
	if len(candidates) != 0 {
		excluded := make(VertexSet)
		bron_kerbosch2_visit(
			graph,
			reporter,
			&candidates,
			&excluded,
			[]Vertex{})
	}
}

func bron_kerbosch2_visit(graph *UndirectedGraph, reporter Reporter, candidates *VertexSet,
	excluded *VertexSet, clique []Vertex) {
	if len(*candidates) == 0 {
		if len(*excluded) == 0 {
			reporter.Record(clique)
		}
		return
	}

	pivot := pick_arbitrary(candidates)
	pivot_neighbours := &graph.adjacencies[pivot]
	far_candidates := make([]Vertex, 0, len(*candidates))
	for c, _ := range *candidates {
		if !contains(pivot_neighbours, c) {
			far_candidates = append(far_candidates, c)
		}
	}
	for _, v := range far_candidates {
		neighbours := &graph.adjacencies[v]
		neighbouring_candidates := intersection(candidates, neighbours)
		neighbouring_excluded := intersection(excluded, neighbours)
		bron_kerbosch2_visit(graph, reporter,
			&neighbouring_candidates,
			&neighbouring_excluded,
			append(clique, v))
		delete(*candidates, v)
		(*excluded)[v] = struct{}{}
	}
}
