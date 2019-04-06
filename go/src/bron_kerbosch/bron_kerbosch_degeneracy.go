package bron_kerbosch

type PriorityQueue struct {
	stack_per_priority [][]Vertex
}

func (q *PriorityQueue) init(max_priority int) {
	q.stack_per_priority = make([][]Vertex, max_priority+1)
}

func (q *PriorityQueue) put(priority int, element Vertex) {
	q.stack_per_priority[priority] = append(q.stack_per_priority[priority], element)
}

func (q *PriorityQueue) pop() Vertex {
	for p := 0; ; p++ {
		l := len(q.stack_per_priority[p])
		if l > 0 {
			last := q.stack_per_priority[p][l-1]
			q.stack_per_priority[p] = q.stack_per_priority[p][:l-1]
			return last
		}
	}
}

func degeneracy_ordering(graph *UndirectedGraph) []Vertex {
	order := graph.order()
	priority_per_node := make([]int, order)
	max_degree := 0
	num_candidates := 0
	for c := 0; c < order; c++ {
		degree := graph.degree(Vertex(c))
		if degree > 0 {
			priority_per_node[Vertex(c)] = degree
			if max_degree < degree {
				max_degree = degree
			}
			num_candidates += 1
		}
	}
	result := make([]Vertex, 0, num_candidates)
	if num_candidates == 0 {
		return result
	}
	// Possible values of priority_per_node:
	//   -1: when yielded
	//   0..max_degree: candidates still queued with priority (degree - #of yielded neighbours)
	var q PriorityQueue
	q.init(max_degree)
	for c, p := range priority_per_node {
		if p > 0 {
			q.put(p, Vertex(c))
		}
	}

	for {
		i := q.pop()
		for priority_per_node[i] == -1 {
			// was requeued with a more urgent priority and therefore already picked
			i = q.pop()
		}
		priority_per_node[i] = -1
		result = append(result, i)
		if len(result) == num_candidates {
			return result
		}
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
