package BronKerbosch

import "sync"

func bronKerbosch3gp2(graph *UndirectedGraph) [][]Vertex {
	return bronKerbosch3om(graph, 5)
}

func bronKerbosch3gp3(graph *UndirectedGraph) [][]Vertex {
	return bronKerbosch3om(graph, 15)
}

func bronKerbosch3gp4(graph *UndirectedGraph) [][]Vertex {
	return bronKerbosch3om(graph, 45)
}

func bronKerbosch3gp5(graph *UndirectedGraph) [][]Vertex {
	return bronKerbosch3om(graph, 135)
}

func bronKerbosch3om(graph *UndirectedGraph, numVisitors int) [][]Vertex {
	// Bron-Kerbosch algorithm with degeneracy ordering, multi-threaded

	starts := make(chan Vertex, numVisitors)
	visits := make(chan VisitJob, numVisitors)
	cliques := make(chan []Vertex)
	go degeneracyOrdering(graph, &ChannelVertexVisitor{starts}, -1)
	go func() {
		excluded := make(VertexSet, graph.connectedVertexCount()-1)
		reporter := ChannelReporter{cliques}
		var wg sync.WaitGroup
		wg.Add(numVisitors)
		for i := 0; i < numVisitors; i++ {
			go func() {
				for job := range visits {
					visit(
						graph, &reporter,
						MaxDegreeLocal, MaxDegreeLocal,
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
		close(reporter.cliques)
	}()
	return gatherCliques(cliques)
}

type VisitJob struct {
	start      Vertex
	candidates VertexSet
	excluded   VertexSet
}
