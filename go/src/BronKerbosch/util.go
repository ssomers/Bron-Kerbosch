package bron_kerbosch

import (
	"math/rand"
)

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
