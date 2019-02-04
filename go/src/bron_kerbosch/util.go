package bron_kerbosch

import (
	"math/rand"
)

func random_choice(vlist *[]Vertex) Vertex {
	i := rand.Intn(len(*vlist))
	return (*vlist)[i]
}

func random_sample(vset *VertexSet) Vertex {
	l := vset.bits.BitLen()
	if l == 0 {
		panic("sampling empty set")
	}
	for {
		i := rand.Intn(l)
		if vset.Contains(Vertex(i)) {
			return Vertex(i)
		}
	}
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
