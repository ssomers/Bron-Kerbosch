package bron_kerbosch

type VertexVisitor interface {
	visit(Vertex)
}

type SimpleVertexVisitor struct {
	vertices []Vertex
}

func (g *SimpleVertexVisitor) visit(v Vertex) {
	g.vertices = append(g.vertices, v)
}

func degeneracy_ordering(graph *UndirectedGraph, visitor VertexVisitor) {
	order := graph.order()
	priority_per_node := make([]int, order)
	max_degree := 0
	num_left_to_visit := 0
	for c := 0; c < order; c++ {
		degree := graph.degree(Vertex(c))
		if degree > 0 {
			priority_per_node[Vertex(c)] = degree
			if max_degree < degree {
				max_degree = degree
			}
			num_left_to_visit += 1
		}
	}
	if num_left_to_visit == 0 {
		return
	}
	// Possible values of priority_per_node:
	//   -1: when yielded
	//   0..max_degree: candidates still queued with priority (degree - #of yielded neighbours)
	var q priority_queue
	q.init(max_degree)
	for c, p := range priority_per_node {
		if p > 0 {
			q.put(p, Vertex(c))
		}
	}

	for {
		i := q.pop()
		for priority_per_node[i] == -1 {
			// was requeued with a more urgent priority and therefore already visited
			i = q.pop()
		}

		visitor.visit(i)
		num_left_to_visit--
		if num_left_to_visit == 0 {
			return
		}

		priority_per_node[i] = -1
		for v := range graph.adjacencies[i] {
			p := priority_per_node[v]
			if p != -1 {
				// Requeue with a more urgent priority, but don't bother to remove
				// the original entry - it will be skipped if it's reached at all.
				priority_per_node[v] = p - 1
				q.put(p-1, v)
			}
		}
	}
}

type priority_queue struct {
	stack_per_priority [][]Vertex
}

func (q *priority_queue) init(max_priority int) {
	q.stack_per_priority = make([][]Vertex, max_priority+1)
}

func (q *priority_queue) put(priority int, element Vertex) {
	q.stack_per_priority[priority] = append(q.stack_per_priority[priority], element)
}

func (q *priority_queue) pop() Vertex {
	for p := 0; ; p++ {
		l := len(q.stack_per_priority[p])
		if l > 0 {
			last := q.stack_per_priority[p][l-1]
			q.stack_per_priority[p] = q.stack_per_priority[p][:l-1]
			return last
		}
	}
}
