package bron_kerbosch

func bron_kerbosch2(graph *UndirectedGraph, reporter Reporter) {
	// Bron-Kerbosch algorithm with pivot picked arbitrarily
	candidates := graph.connected_nodes()
	if !candidates.IsEmpty() {
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
	if candidates.IsEmpty() {
		if excluded.IsEmpty() {
			reporter.Record(clique)
		}
		return
	}

	pivot := candidates.PickArbitrary()
	pivot_neighbours := &graph.adjacencies[pivot]
	far_candidates := make([]Vertex, 0, len(*candidates))
	for c, _ := range *candidates {
		if !pivot_neighbours.Contains(c) {
			far_candidates = append(far_candidates, c)
		}
	}
	for _, v := range far_candidates {
		candidates.Remove(v)
		neighbours := &graph.adjacencies[v]
		neighbouring_candidates := candidates.Intersection(neighbours)
		neighbouring_excluded := excluded.Intersection(neighbours)
		bron_kerbosch2_visit(graph, reporter,
			&neighbouring_candidates,
			&neighbouring_excluded,
			append(clique, v))
		excluded.Add(v)
	}
}
