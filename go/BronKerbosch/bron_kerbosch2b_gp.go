package BronKerbosch

func bronKerbosch2bGP(graph *UndirectedGraph, consumer Consumer) {
	// Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
	// chosen from candidates only (IK_GP).
	order := graph.Order()
	if order > 0 {
		pivot := graph.maxDegreeVertex()
		// In this initial iteration, we don't need to represent the set of candidates
		// because all neighbours are candidates until excluded.
		excluded := make([]bool, order)
		for i := range order {
			v := Vertex(i)
			neighbours := graph.neighbours(v)
			if !neighbours.IsEmpty() && !neighbours.Contains(pivot) {
				neighbouringExcluded := neighbours.IntersectionWithMap(excluded)
				if len(neighbouringExcluded) < len(neighbours) {
					neighbouringCandidates := neighbours.Difference(neighbouringExcluded)
					visit(
						graph, consumer,
						MaxDegreeLocal,
						neighbouringCandidates,
						neighbouringExcluded,
						[]Vertex{v})
				}
				excluded[v] = true
			}
		}
	}
}
