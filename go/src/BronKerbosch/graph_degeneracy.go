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

func degeneracyOrdering(graph *UndirectedGraph, visitor VertexVisitor, drop int) {
	if drop > 0 {
		panic("expecting negative drop value")
	}
	defer func() { visitor.Close() }()
	order := graph.order()
	// Possible values of priorityPerNode:
	//   -1: when yielded
	//   0..maxDegree: candidates still queued with priority (degree - #of yielded neighbours)
	priorityPerNode := make([]int, order)
	maxDegree := 0
	numLeftToVisit := 0
	for c := 0; c < order; c++ {
		degree := graph.degree(Vertex(c))
		if degree > 0 {
			priorityPerNode[Vertex(c)] = degree
			if maxDegree < degree {
				maxDegree = degree
			}
			numLeftToVisit++
		}
	}
	numLeftToVisit += drop
	if numLeftToVisit <= 0 {
		return
	}

	var q priorityQueue
	q.init(maxDegree)
	for c, p := range priorityPerNode {
		if p > 0 {
			q.put(p, Vertex(c))
		}
	}

	for {
		i := q.pop()
		for priorityPerNode[i] == -1 {
			// was requeued with a more urgent priority and therefore already visited
			i = q.pop()
		}

		visitor.visit(i)
		numLeftToVisit--
		if numLeftToVisit == 0 {
			return
		}

		priorityPerNode[i] = -1
		for v := range graph.adjacencies[i] {
			p := priorityPerNode[v]
			if p != -1 {
				// Requeue with a more urgent priority, but don't bother to remove
				// the original entry - it will be skipped if it's reached at all.
				priorityPerNode[v] = p - 1
				q.put(p-1, v)
			}
		}
	}
}

type priorityQueue struct {
	stackPerPriority [][]Vertex
}

func (q *priorityQueue) init(maxPriority int) {
	q.stackPerPriority = make([][]Vertex, maxPriority+1)
}

func (q *priorityQueue) put(priority int, element Vertex) {
	q.stackPerPriority[priority] = append(q.stackPerPriority[priority], element)
}

func (q *priorityQueue) pop() Vertex {
	for p := 0; ; p++ {
		l := len(q.stackPerPriority[p])
		if l > 0 {
			last := q.stackPerPriority[p][l-1]
			q.stackPerPriority[p] = q.stackPerPriority[p][:l-1]
			return last
		}
	}
}
