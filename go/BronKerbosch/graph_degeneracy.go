package BronKerbosch

type DegeneracyVisitItem struct {
	pick             Vertex
	pickedNeighbours VertexSet
}

type DegeneracyVisitor interface {
	visit(DegeneracyVisitItem)
	Close()
}

type SimpleDegeneracyVisitor struct {
	vertices []Vertex
}

type ChannelDegeneracyVisitor struct {
	vertices chan<- DegeneracyVisitItem
}

func (g *SimpleDegeneracyVisitor) visit(i DegeneracyVisitItem) {
	g.vertices = append(g.vertices, i.pick)
}

func (g *SimpleDegeneracyVisitor) Close() {
}

func (g *ChannelDegeneracyVisitor) visit(i DegeneracyVisitItem) {
	g.vertices <- i
}

func (g *ChannelDegeneracyVisitor) Close() {
	close(g.vertices)
}

func degeneracyVisitor(graph *UndirectedGraph, visitor DegeneracyVisitor) {
	defer func() { visitor.Close() }()
	order := graph.Order()
	// Possible values of priorityPerVertex:
	//   0: when yielded
	//   1..maxDegree: candidates still queued with priority (degree - #of yielded neighbours)
	priorityPerVertex := make([]int, order)
	numLeftToPick := 0
	var q priorityQueue[Vertex]
	q.init(graph.max_degree)
	for i := range order {
		v := Vertex(i)
		priority := graph.degree(v)
		if priority > 0 {
			priorityPerVertex[v] = priority
			numLeftToPick++
			q.put(v, priority)
		}
	}

	for numLeftToPick > 0 {
		i := DegeneracyVisitItem{}
		i.pick = q.pop()
		if priorityPerVertex[i.pick] > 0 {
			priorityPerVertex[i.pick] = 0
			i.pickedNeighbours = make(VertexSet, graph.degree(i.pick))
			for v := range graph.neighbours(i.pick) {
				oldPriority := priorityPerVertex[v]
				if oldPriority > 0 {
					newPriority := oldPriority - 1
					priorityPerVertex[v] = newPriority
					// Requeue with a more urgent priority or dequeue.
					// Don't bother to remove the original entry from the queue,
					// since the vertex will be skipped when popped, and thanks to
					// numLeftToPick we might not need to pop it at all.
					if newPriority > 0 {
						q.put(v, newPriority)
					} else {
						// We discount this neighbour already, but logically it will
						// be (silently) picked only after we yield the current pick.
						// So it does not belong in the current pickedNeighbours.
						numLeftToPick--
					}
				} else {
					i.pickedNeighbours.Add(v)
				}
			}
			visitor.visit(i)
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
