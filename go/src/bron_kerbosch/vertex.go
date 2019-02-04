package bron_kerbosch

import (
	"math/big"
)

type Vertex int
type VertexSet struct {
	bits big.Int
}

func NewVertexSet(vertices []Vertex) VertexSet {
	var r VertexSet
	for _, v := range vertices {
		r.Add(v)
	}
	return r
}

func (vset *VertexSet) IsEmpty() bool {
	return vset.bits.Cmp(big.NewInt(0)) == 0
}

func (vset *VertexSet) Cardinality() int {
	var r uint
	for l := vset.bits.BitLen() - 1; l >= 0; l-- {
		r += vset.bits.Bit(l)
	}
	return int(r)
}

func (vset *VertexSet) Iterate() []Vertex {
	var r []Vertex
	for l := vset.bits.BitLen() - 1; l >= 0; l-- {
		if vset.bits.Bit(l) != 0 {
			r = append(r, Vertex(l))
		}
	}
	return r
}

func (vset *VertexSet) Contains(v Vertex) bool {
	return vset.bits.Bit(int(v)) != 0
}

func (vset1 *VertexSet) Intersection(vset2 *VertexSet) VertexSet {
	var r VertexSet
	r.bits.And(&vset1.bits, &vset2.bits)
	return r
}

func (vset *VertexSet) Add(v Vertex) {
	vset.bits.SetBit(&vset.bits, int(v), 1)
}

func (vset *VertexSet) Remove(v Vertex) {
	vset.bits.SetBit(&vset.bits, int(v), 0)
}

func (vset *VertexSet) PickArbitrary() Vertex {
	l := vset.bits.BitLen() - 1
	if l < 0 {
		panic("picking from empty set")
	}
	if vset.bits.Bit(l) != 1 {
		panic("eek")
	}
	return Vertex(l)
}

func (vset *VertexSet) PopArbitrary() Vertex {
	v := vset.PickArbitrary()
	vset.Remove(v)
	return v
}
