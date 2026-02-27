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
	var q priorityQueue[Vertex]
	q.init(graph.max_degree)
	for v := range order {
		priority := graph.degree(Vertex(v))
		priorityPerNode[Vertex(v)] = priority
		q.insert(Vertex(v), priority)
	}

	for !q.empty() {
		pick := q.pop()
		if priorityPerNode[pick] > 0 {
			priorityPerNode[pick] = 0
			visitor.visit(pick)
			q.forget(pick)
			for v := range graph.neighbours(pick) {
				oldPriority := priorityPerNode[v]
				if oldPriority > 0 {
					newPriority := oldPriority - 1
					priorityPerNode[v] = newPriority
					q.promote(v, newPriority)
				}
			}
		}
	}
}

type priorityQueue[T interface{}] struct {
	stackPerPriority [][]T
	numLeftToPick    int
}

func (q *priorityQueue[T]) empty() bool {
	return q.numLeftToPick == 0
}

func (q *priorityQueue[T]) init(maxPriority int) {
	q.stackPerPriority = make([][]T, maxPriority)
	q.numLeftToPick = 0
}

func (q *priorityQueue[T]) insert(element T, priority int) {
	if priority > 0 {
		q.stackPerPriority[priority-1] = append(q.stackPerPriority[priority-1], element)
		q.numLeftToPick += 1
	}
}

// Requeue with a more urgent priority or unqueue.
// Don't bother to remove the original entry from the queue,
// since the vertex will be skipped when popped, and thanks to
// numLeftToVisit we might not need to pop it at all.
func (q *priorityQueue[T]) promote(element T, priority int) {
	if priority > 0 {
		q.stackPerPriority[priority-1] = append(q.stackPerPriority[priority-1], element)
	} else {
		q.forget(element)
	}
}

func (q *priorityQueue[T]) forget(element T) {
	if q.numLeftToPick == 0 {
		panic("attempt to forget what was forgotten")
	}
	q.numLeftToPick -= 1
}

// We may return an element already popped, even though it was passed to forget,
// in case its priority was promoted earlier on. That's why we do not count 
// the element as picked, but wait for the caller to forget it. The caller must
// somehow ensure to forget the same element only once.
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
