package BronKerbosch

// Bron-Kerbosch algorithm with degeneracy ordering, multi-threaded

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
	starts := make(chan DegeneracyVisitItem, numVisitors)
	visits := make(chan VisitJob, numVisitors)
	go degeneracyVisitor(graph, &ChannelDegeneracyVisitor{starts})
	go func() {
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
		for i := range starts {
			v := i.pick
			neighbouringExcluded := i.pickedNeighbours
			neighbours := graph.neighbours(v)
			if len(neighbouringExcluded) < len(neighbours) {
				neighbouringCandidates := neighbours.Difference(neighbouringExcluded)
				visits <- VisitJob{v, neighbouringCandidates, neighbouringExcluded}
			}
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
