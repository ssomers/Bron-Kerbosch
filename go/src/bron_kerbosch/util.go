package bron_kerbosch

import (
	"math/rand"
)

func contains(vset *VertexSet, v Vertex) bool {
	_, ok := (*vset)[v]
	return ok
}

func intersection(vset1 *VertexSet, vset2 *VertexSet) VertexSet {
	result := make(VertexSet)
	if vset1 != nil && vset2 != nil {
		if len(*vset1) > len(*vset2) {
			vset1, vset2 = vset2, vset1
		}
		for v, _ := range *vset1 {
			if contains(vset2, v) {
				result[v] = struct{}{}
			}
		}
	}
	return result
}

func pop_arbitrary(vset *VertexSet) Vertex {
	for v, _ := range *vset {
		delete(*vset, v)
		return v
	}
	panic("attempt to pop from empty set")
}

func random_choice(vlist *[]Vertex) Vertex {
	i := rand.Intn(len(*vlist))
	return (*vlist)[i]
}

func random_sample(vset *VertexSet) Vertex {
	i := rand.Intn(len(*vset))
	for v, _ := range *vset {
		if i == 0 {
			return v
		}
		i -= 1
	}
	panic("should have returned")
}

func array_remove(vlist []Vertex, doomed Vertex) []Vertex {
	for i, v := range vlist {
		if v == doomed {
			l := len(vlist)
			vlist[i] = vlist[l-1]
			return vlist[:l-1]
		}
	}
	panic("not found")
}
