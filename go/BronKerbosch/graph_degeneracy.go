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

func degeneracyVisitor(graph *UndirectedGraph, visitor VertexVisitor) {
	defer func() { visitor.Close() }()
	order := graph.Order()
	// Possible values of priorityPerNode:
	//   0: when yielded
	//   1..maxDegree: candidates still queued with priority (degree - #of yielded neighbours)
	priorityPerNode := make([]int, order)
	numLeftToPick := 0
	var q priorityQueue[Vertex]
	q.init(graph.max_degree)
	for v := range order {
		priority := graph.degree(Vertex(v))
		if priority > 0 {
			priorityPerNode[Vertex(v)] = priority
			numLeftToPick++
			q.put(Vertex(v), priority)
		}
	}

	for numLeftToPick > 0 {
		pick := q.pop()
		if priorityPerNode[pick] > 0 {
			priorityPerNode[pick] = 0
			visitor.visit(pick)
			for v := range graph.neighbours(pick) {
				oldPriority := priorityPerNode[v]
				if oldPriority > 0 {
					newPriority := oldPriority - 1
					priorityPerNode[v] = newPriority
					// Requeue with a more urgent priority or dequeue.
					// Don't bother to remove the original entry from the queue,
					// since the vertex will be skipped when popped, and thanks to
					// numLeftToPick we might not need to pop it at all.
					if newPriority > 0 {
						q.put(v, newPriority)
					} else {
						numLeftToPick--
					}
				}
			}
			numLeftToPick--
			if numLeftToPick < 0 {
				panic("numLeftToPick < 0")
			}
		}
	}
}

type priorityQueue[T interface{}] struct {
	stackPerPriority [][]T
}

func (q *priorityQueue[T]) init(maxPriority int) {
	q.stackPerPriority = make([][]T, maxPriority)
}

func (q *priorityQueue[T]) put(element T, priority int) {
	q.stackPerPriority[priority-1] = append(q.stackPerPriority[priority-1], element)
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
