package BronKerbosch

func bronKerbosch2bGP(graph *UndirectedGraph, reporter Reporter) {
	// Bron-Kerbosch algorithm with pivot of highest degree within remaining candidates
	// chosen from candidates only (IK_GP).
	order := graph.Order()
	if order == 0 {
		return
	}
	pivot := graph.maxDegreeVertex()
	excluded := make(VertexSet, order)
	for i := range order {
		v := Vertex(i)
		neighbours := graph.neighbours(v)
		if !neighbours.Contains(pivot) {
			neighbouringExcluded := neighbours.Intersection(excluded)
			if len(neighbouringExcluded) < len(neighbours) {
				neighbouringCandidates := neighbours.Difference(neighbouringExcluded)
				visit(
					graph, reporter,
					MaxDegreeLocal,
					neighbouringCandidates,
					neighbouringExcluded,
					[]Vertex{v})
			}
			excluded.Add(v)
		}
	}
}
