package BronKerbosch

// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP).

func bronKerbosch3gp(graph *UndirectedGraph, consumer Consumer) {
	degeneracyVisitor(graph, func(i DegeneracyVisitItem) {
		v := i.pick
		neighbouringCandidates, neighbouringExcluded :=
			graph.neighbours(v).Partition(i.isCandidate)
		visit(
			graph, consumer,
			MaxDegreeLocal,
			neighbouringCandidates,
			neighbouringExcluded,
			[]Vertex{v})
	})
}
