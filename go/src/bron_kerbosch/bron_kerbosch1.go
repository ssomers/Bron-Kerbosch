package bron_kerbosch

func bron_kerbosch1(graph *UndirectedGraph, reporter Reporter) {
	// Naive Bron-Kerbosch algorithm
	candidates := graph.connected_vertices()
	if !candidates.IsEmpty() {
		excluded := make(VertexSet, len(candidates))
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
	for {
		v := candidates.PopArbitrary()
		neighbours := &graph.adjacencies[v]
		neighbouring_candidates := candidates.Intersection(neighbours)
		if !neighbouring_candidates.IsEmpty() {
			neighbouring_excluded := excluded.Intersection(neighbours)
			bron_kerbosch1_visit(graph, reporter,
				&neighbouring_candidates,
				&neighbouring_excluded,
				append(clique, v))
		} else {
			if excluded.IsDisjoint(neighbours) {
				reporter.Record(append(clique, v))
			}
			if candidates.IsEmpty() {
				break
			}
		}
		excluded.Add(v)
	}
}
