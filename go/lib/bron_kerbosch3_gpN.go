package BronKerbosch

import "sync"

func bronKerbosch3gp0(graph *UndirectedGraph, reporter Reporter) {
	bronKerbosch3om(graph, reporter, 1)
}

func bronKerbosch3gp1(graph *UndirectedGraph, reporter Reporter) {
	bronKerbosch3om(graph, reporter, 4)
}

func bronKerbosch3gp2(graph *UndirectedGraph, reporter Reporter) {
	bronKerbosch3om(graph, reporter, 16)
}

func bronKerbosch3gp3(graph *UndirectedGraph, reporter Reporter) {
	bronKerbosch3om(graph, reporter, 64)
}

func bronKerbosch3gp4(graph *UndirectedGraph, reporter Reporter) {
	bronKerbosch3om(graph, reporter, 256)
}

func bronKerbosch3om(graph *UndirectedGraph, finalReporter Reporter, numVisitors int) {
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
		close(reporter.cliques)
	}()
	for clique := range cliques {
		finalReporter.Record(clique)
	}
}

type VisitJob struct {
	start      Vertex
	candidates VertexSet
	excluded   VertexSet
}
