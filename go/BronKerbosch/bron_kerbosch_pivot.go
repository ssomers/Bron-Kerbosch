package BronKerbosch

type PivotSelection int

const (
	MaxDegreeLocal  PivotSelection = iota
	MaxDegreeLocalX                = iota
)

func visit(graph *UndirectedGraph, reporter Reporter,
	pivotSelection PivotSelection,
	candidates VertexSet, excluded VertexSet, clique []Vertex) {
	if len(candidates) == 1 {
		for v := range candidates {
			// Same logic as below, stripped down
			neighbours := graph.neighbours(v)
			if excluded.IsDisjoint(neighbours) {
				reporter.Record(append(clique, v))
			}
		}
		return
	}

	var pivot Vertex
	remainingCandidates := make([]Vertex, 0, len(candidates))
	// Quickly handle locally unconnected candidates while finding pivot
	seenLocalDegree := 0
	for v := range candidates {
		neighbours := graph.neighbours(v)
		localDegree := neighbours.IntersectionLen(candidates)
		if localDegree == 0 {
			// Same logic as below, stripped down
			if neighbours.IsDisjoint(excluded) {
				reporter.Record(append(clique, v))
			}
		} else {
			if seenLocalDegree < localDegree {
				seenLocalDegree = localDegree
				pivot = v
			}
			remainingCandidates = append(remainingCandidates, v)
		}
	}
	if seenLocalDegree == 0 {
		return
	}
	if pivotSelection == MaxDegreeLocalX {
		for v := range excluded {
			neighbours := graph.neighbours(v)
			localDegree := neighbours.IntersectionLen(candidates)
			if seenLocalDegree < localDegree {
				seenLocalDegree = localDegree
				pivot = v
			}
		}
	}

	for _, v := range remainingCandidates {
		neighbours := graph.neighbours(v)
		if neighbours.Contains(pivot) {
			continue
		}
		candidates.Remove(v)
		neighbouringCandidates := neighbours.Intersection(candidates)
		if !neighbouringCandidates.IsEmpty() {
			neighbouringExcluded := neighbours.Intersection(excluded)
			visit(
				graph, reporter,
				pivotSelection,
				neighbouringCandidates,
				neighbouringExcluded,
				append(clique, v))
		} else {
			if neighbours.IsDisjoint(excluded) {
				reporter.Record(append(clique, v))
			}
		}
		excluded.Add(v)
	}
}
