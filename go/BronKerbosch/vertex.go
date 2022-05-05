package BronKerbosch

type Vertex int
type VertexSet map[Vertex]struct{}

func NewVertexSet(vertices []Vertex) VertexSet {
	r := make(VertexSet)
	for _, v := range vertices {
		r.Add(v)
	}
	return r
}

func (vset VertexSet) IsEmpty() bool {
	return len(vset) == 0
}

func (vset VertexSet) Cardinality() int {
	return len(vset)
}

func (vset VertexSet) Contains(v Vertex) bool {
	_, ok := vset[v]
	return ok
}

func (vset VertexSet) Difference(term VertexSet) VertexSet {
	result := make(VertexSet, len(vset))
	for v := range vset {
		if !term.Contains(v) {
			result.Add(v)
		}
	}
	return result
}

func (vset VertexSet) Intersection(other VertexSet) VertexSet {
	small, large := vset, other
	if len(small) > len(large) {
		small, large = other, vset
	}
	result := make(VertexSet, len(small))
	for v := range small {
		if large.Contains(v) {
			result.Add(v)
		}
	}
	return result
}

func (vset VertexSet) IntersectionLen(other VertexSet) int {
	small, large := vset, other
	if len(small) > len(large) {
		small, large = other, vset
	}
	result := 0
	for v := range small {
		if large.Contains(v) {
			result++
		}
	}
	return result
}

func (vset VertexSet) IsDisjoint(other VertexSet) bool {
	small, large := vset, other
	if len(small) > len(large) {
		small, large = other, vset
	}
	for v := range small {
		if large.Contains(v) {
			return false
		}
	}
	return true
}

func (vset VertexSet) Add(v Vertex) {
	vset[v] = struct{}{}
}

func (vset VertexSet) Remove(v Vertex) {
	delete(vset, v)
}

func (vset VertexSet) PickArbitrary() Vertex {
	for v := range vset {
		return v
	}
	panic("attempt to pick from empty set")
}

func (vset VertexSet) PopArbitrary() Vertex {
	for v := range vset {
		vset.Remove(v)
		return v
	}
	panic("attempt to pop from empty set")
}
