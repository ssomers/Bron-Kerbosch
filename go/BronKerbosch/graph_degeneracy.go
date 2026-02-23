package BronKerbosch

type VertexVisitor interface {
	visit(Vertex)
	Close()
}

type SimpleVertexVisitor struct {
	vertices []Vertex
}

type ChannelVertexVisitor struct {
	vertices chan<- Vertex
}

func (g *SimpleVertexVisitor) visit(v Vertex) {
	g.vertices = append(g.vertices, v)
}

func (g *SimpleVertexVisitor) Close() {
}

func (g *ChannelVertexVisitor) visit(v Vertex) {
	g.vertices <- v
}

func (g *ChannelVertexVisitor) Close() {
	close(g.vertices)
}

func degeneracyOrdering(graph *UndirectedGraph, visitor VertexVisitor) {
	defer func() { visitor.Close() }()
	order := graph.Order()
	// Possible values of priorityPerNode:
	//   -1: when yielded
	//   0..maxDegree: candidates still queued with priority (degree - #of yielded neighbours)
	priorityPerNode := make([]int, order)
	maxDegree := 0
	for v := range order {
		degree := graph.degree(Vertex(v))
		if degree > 0 {
			priorityPerNode[Vertex(v)] = degree
			if maxDegree < degree {
				maxDegree = degree
			}
		}
	}

	numLeftToVisit := 0
	var q priorityQueue[Vertex]
	q.init(maxDegree)
	for v, priority := range priorityPerNode {
		if priority > 0 {
			q.put(priority, Vertex(v))
			numLeftToVisit++
		}
	}

	for numLeftToVisit > 0 {
		pick := q.pop()
		if priorityPerNode[pick] > 0 {
			visitor.visit(pick)
			priorityPerNode[pick] = 0
			numLeftToVisit--
			for v := range graph.neighbours(pick) {
				oldPriority := priorityPerNode[v]
				if oldPriority > 0 {
					// Requeue with a more urgent priority or unqueue.
					// Don't bother to remove the original entry from the queue,
					// since the vertex will be skipped when popped, and thanks to
					// numLeftToVisit we might not need to pop it at all.
					newPriority := oldPriority - 1
					priorityPerNode[v] = newPriority
					if newPriority > 0 {
						q.put(newPriority, v)
					} else {
						numLeftToVisit--
					}
				}
			}
		}
	}
}

type priorityQueue[T interface{}] struct {
	stackPerPriority [][]T
}

func (q *priorityQueue[T]) init(maxPriority int) {
	q.stackPerPriority = make([][]T, maxPriority+1)
}

func (q *priorityQueue[T]) put(priority int, element T) {
	q.stackPerPriority[priority] = append(q.stackPerPriority[priority], element)
}

func (q *priorityQueue[T]) pop() T {
	for p := 0; ; p++ {
		l := len(q.stackPerPriority[p]) // IndexError when attempting to pop more than was put
		if l > 0 {
			last := q.stackPerPriority[p][l-1]
			q.stackPerPriority[p] = q.stackPerPriority[p][:l-1]
			return last
		}
	}
}
