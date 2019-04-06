package bron_kerbosch

func bron_kerbosch3(graph *UndirectedGraph, reporter Reporter) {
	// Bron-Kerbosch algorithm with degeneracy ordering
	candidates := graph.connected_vertices()
	excluded := make(VertexSet, len(candidates))
	for _, v := range degeneracy_ordering(graph) {
		neighbours := graph.adjacencies[v]
		candidates.Remove(v)
		neighbouring_candidates := candidates.Intersection(&neighbours)
		if !neighbouring_candidates.IsEmpty() {
			neighbouring_excluded := excluded.Intersection(&neighbours)
			visit_max_degree(
				graph,
				reporter,
				&neighbouring_candidates,
				&neighbouring_excluded,
				[]Vertex{v})
		}
		excluded.Add(v)
	}
}
