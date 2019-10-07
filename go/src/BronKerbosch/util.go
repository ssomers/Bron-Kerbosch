package BronKerbosch

import (
	"math/rand"
)

func randomChoice(vlist *[]Vertex) Vertex {
	i := rand.Intn(len(*vlist))
	return (*vlist)[i]
}

func randomSample(vset *VertexSet) Vertex {
	i := rand.Intn(len(*vset))
	for v := range *vset {
		if i == 0 {
			return v
		}
		i--
	}
	panic("should have returned")
}

func removeFromArray(vlist []Vertex, doomed Vertex) []Vertex {
	for i, v := range vlist {
		if v == doomed {
			l := len(vlist)
			vlist[i] = vlist[l-1]
			return vlist[:l-1]
		}
	}
	panic("not found")
}
