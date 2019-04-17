package bron_kerbosch

type PivotSelection int

const (
	MaxDegree      PivotSelection = iota
	MaxDegreeLocal                = iota
)

func visit(graph *UndirectedGraph, reporter Reporter,
	initial_pivot_selection PivotSelection, further_pivot_selection PivotSelection,
	candidates VertexSet, excluded VertexSet, clique []Vertex) {
	if len(candidates) == 1 {
		for v := range candidates {
			neighbours := graph.adjacencies[v]
			if excluded.IsDisjoint(neighbours) {
				reporter.Record(append(clique, v))
			}
		}
		return
	}

	var pivot Vertex
	remaining_candidates := make([]Vertex, 0, len(candidates))
	switch initial_pivot_selection {
	case MaxDegree:
		pivot = pick_max_degree(graph, candidates, excluded)
		for v, _ := range candidates {
			remaining_candidates = append(remaining_candidates, v)
		}
		break
	case MaxDegreeLocal:
		// Quickly handle locally unconnected candidates while finding pivot
		var seen_local_degree = 0
		for v := range candidates {
			neighbours := graph.adjacencies[v]
			local_degree := neighbours.IntersectionLen(candidates)
			if local_degree == 0 {
				// Same logic as below, but stripped down
				if neighbours.IsDisjoint(excluded) {
					reporter.Record(append(clique, v))
				}
			} else {
				if seen_local_degree < local_degree {
					seen_local_degree = local_degree
					pivot = v
				}
				remaining_candidates = append(remaining_candidates, v)
			}
		}
		if len(remaining_candidates) == 0 {
			return
		}
		for v := range excluded {
			neighbours := graph.adjacencies[v]
			local_degree := neighbours.IntersectionLen(candidates)
			if seen_local_degree < local_degree {
				seen_local_degree = local_degree
				pivot = v
			}
		}
		break
	}

	for _, v := range remaining_candidates {
		neighbours := graph.adjacencies[v]
		if neighbours.Contains(pivot) {
			continue
		}
		candidates.Remove(v)
		neighbouring_candidates := neighbours.Intersection(candidates)
		if !neighbouring_candidates.IsEmpty() {
			neighbouring_excluded := neighbours.Intersection(excluded)
			visit(
				graph, reporter,
				further_pivot_selection, further_pivot_selection,
				neighbouring_candidates,
				neighbouring_excluded,
				append(clique, v))
		} else {
			if neighbours.IsDisjoint(excluded) {
				reporter.Record(append(clique, v))
			}
		}
		excluded.Add(v)
	}
}

func pick_max_degree(graph *UndirectedGraph, candidates VertexSet, excluded VertexSet) Vertex {
	max_degree := 0
	var max_vertex Vertex
	for v, _ := range candidates {
		degree := graph.degree(v)
		if max_degree < degree {
			max_degree = degree
			max_vertex = v
		}
	}
	return max_vertex
}
