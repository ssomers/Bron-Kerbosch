package bron_kerbosch

func bron_kerbosch2(graph *UndirectedGraph, reporter Reporter) {
	// Bron-Kerbosch algorithm with pivot of highest degree
	candidates := graph.connected_vertices()
	if !candidates.IsEmpty() {
		excluded := make(VertexSet, len(candidates))
		visit_max_degree(
			graph,
			reporter,
			&candidates,
			&excluded,
			[]Vertex{})
	}
}
