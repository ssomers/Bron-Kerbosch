package bron_kerbosch

func visit_max_degree(graph *UndirectedGraph, reporter Reporter,
	candidates *VertexSet, excluded *VertexSet, clique []Vertex) {
	pivot := pick_max_degree(graph, candidates, excluded)
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
		if !neighbouring_candidates.IsEmpty() {
			neighbouring_excluded := excluded.Intersection(neighbours)
			visit_max_degree(graph, reporter,
				&neighbouring_candidates,
				&neighbouring_excluded,
				append(clique, v))
		} else {
			if excluded.IsDisjoint(neighbours) {
				reporter.Record(append(clique, v))
			}
		}
		excluded.Add(v)
	}
}

func pick_max_degree(graph *UndirectedGraph, candidates *VertexSet, excluded *VertexSet) Vertex {
	max_degree := 0
	var max_vertex Vertex
	for v, _ := range *candidates {
		degree := graph.degree(v)
		if max_degree < degree {
			max_degree = degree
			max_vertex = v
		}
	}
	return max_vertex
}
