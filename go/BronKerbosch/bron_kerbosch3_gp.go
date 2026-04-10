package BronKerbosch

// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP).

func bronKerbosch3gp(graph *UndirectedGraph, consumer Consumer) {
	// In this initial iteration, we don't need to represent the set of candidates
	// because all neighbours are candidates until excluded.
	degeneracyVisitor(graph, func(i DegeneracyVisitItem) {
		v := i.pick
		neighbouringExcluded := i.pickedNeighbours
		neighbouringCandidates := graph.neighbours(v).Difference(neighbouringExcluded)
		visit(
			graph, consumer,
			MaxDegreeLocal,
			neighbouringCandidates,
			neighbouringExcluded,
			[]Vertex{v})
	})
	consumer.close()
}
