package BronKerbosch

import "sync"

func bronKerbosch3gp0(graph *UndirectedGraph, cliques chan<- []Vertex) {
	bronKerbosch3om(graph, cliques, 1)
}

func bronKerbosch3gp1(graph *UndirectedGraph, cliques chan<- []Vertex) {
	bronKerbosch3om(graph, cliques, 4)
}

func bronKerbosch3gp2(graph *UndirectedGraph, cliques chan<- []Vertex) {
	bronKerbosch3om(graph, cliques, 16)
}

func bronKerbosch3gp3(graph *UndirectedGraph, cliques chan<- []Vertex) {
	bronKerbosch3om(graph, cliques, 64)
}

func bronKerbosch3gp4(graph *UndirectedGraph, cliques chan<- []Vertex) {
	bronKerbosch3om(graph, cliques, 256)
}

func bronKerbosch3om(graph *UndirectedGraph, cliques chan<- []Vertex, numVisitors int) {
	// Bron-Kerbosch algorithm with degeneracy ordering, multi-threaded

	starts := make(chan Vertex, numVisitors)
	visits := make(chan VisitJob, numVisitors)
	go degeneracyVisitor(graph, &ChannelVertexVisitor{starts})
	go func() {
		excluded := make(VertexSet, graph.connectedVertexCount-1)
		var wg sync.WaitGroup
		wg.Add(numVisitors)
		for range numVisitors {
			go func() {
				for job := range visits {
					visit(
						graph, cliques,
						MaxDegreeLocal,
						job.candidates,
						job.excluded,
						[]Vertex{job.start})
				}
				wg.Done()
			}()
		}
		for v := range starts {
			neighbours := graph.neighbours(v)
			neighbouringCandidates := neighbours.Difference(excluded)
			if !neighbouringCandidates.IsEmpty() {
				neighbouringExcluded := neighbours.Intersection(excluded)
				visits <- VisitJob{v, neighbouringCandidates, neighbouringExcluded}
			}
			excluded.Add(v)
		}
		close(visits)
		wg.Wait()
		close(cliques)
	}()
}

type VisitJob struct {
	start      Vertex
	candidates VertexSet
	excluded   VertexSet
}
