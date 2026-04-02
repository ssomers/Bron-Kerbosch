package BronKerbosch

type PivotSelection int

const (
	MaxDegreeLocal  PivotSelection = iota
	MaxDegreeLocalX                = iota
)

func visit(graph *UndirectedGraph, consumer Consumer,
	pivotSelection PivotSelection,
	candidates VertexSet, excluded VertexSet, cliqueInProgress []Vertex) {
	switch len(candidates) {
	case 0:
		return
	case 1:
		for v := range candidates {
			// Same logic as below, stripped down
			neighbours := graph.neighbours(v)
			if len(cliqueInProgress)+1 >= consumer.MinSize && excluded.IsDisjoint(neighbours) {
				consumer.Add(Append(cliqueInProgress, v))
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
			if len(cliqueInProgress)+1 >= consumer.MinSize && neighbours.IsDisjoint(excluded) {
				consumer.Add(Append(cliqueInProgress, v))
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
				graph, consumer,
				pivotSelection,
				neighbouringCandidates,
				neighbouringExcluded,
				append(cliqueInProgress, v))
		} else {
			if len(cliqueInProgress)+1 >= consumer.MinSize && neighbours.IsDisjoint(excluded) {
				consumer.Add(Append(cliqueInProgress, v))
			}
		}
		excluded.Add(v)
	}
}
