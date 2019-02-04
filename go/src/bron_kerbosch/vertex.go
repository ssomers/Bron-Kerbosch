package bron_kerbosch

type Vertex int
type VertexSet map[Vertex]struct{}

func NewVertexSet(vertices []Vertex) VertexSet {
	r := make(VertexSet)
	for _, v := range vertices {
		r.Add(v)
	}
	return r
}

func (vset *VertexSet) IsEmpty() bool {
	return len(*vset) == 0
}

func (vset *VertexSet) Cardinality() int {
	return len(*vset)
}

func (vset *VertexSet) Contains(v Vertex) bool {
	_, ok := (*vset)[v]
	return ok
}

func (vset1 *VertexSet) Intersection(vset2 *VertexSet) VertexSet {
	result := make(VertexSet)
	if vset1 != nil && vset2 != nil {
		if len(*vset1) > len(*vset2) {
			vset1, vset2 = vset2, vset1
		}
		for v, _ := range *vset1 {
			if vset2.Contains(v) {
				result.Add(v)
			}
		}
	}
	return result
}

func (vset *VertexSet) Add(v Vertex) {
	(*vset)[v] = struct{}{}
}

func (vset *VertexSet) Remove(v Vertex) {
	delete(*vset, v)
}

func (vset *VertexSet) PickArbitrary() Vertex {
	for v, _ := range *vset {
		return v
	}
	panic("attempt to pick from empty set")
}

func (vset *VertexSet) PopArbitrary() Vertex {
	v := vset.PickArbitrary()
	vset.Remove(v)
	return v
}
