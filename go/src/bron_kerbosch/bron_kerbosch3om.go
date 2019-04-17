package bron_kerbosch

import "sync"

const NUM_VISITORS = 8

func bron_kerbosch3om(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with degeneracy ordering, multi-threaded

	starts := make(chan Vertex, NUM_VISITORS)
	visits := make(chan VisitJob, NUM_VISITORS)
	cliques := make(chan []Vertex)
	go degeneracy_ordering(graph, &ChannelVertexVisitor{starts}, -1)
	go func() {
		excluded := make(VertexSet, graph.connected_vertex_count()-1)
		reporter := ChannelReporter{cliques}
		var wg sync.WaitGroup
		wg.Add(NUM_VISITORS)
		for i := 0; i < NUM_VISITORS; i++ {
			go func() {
				for job := range visits {
					visit(
						graph, &reporter,
						MaxDegree, MaxDegree,
						job.candidates,
						job.excluded,
						[]Vertex{job.start})
				}
				wg.Done()
			}()
		}
		for v := range starts {
			neighbours := graph.adjacencies[v]
			neighbouring_candidates := neighbours.Difference(excluded)
			if !neighbouring_candidates.IsEmpty() {
				neighbouring_excluded := neighbours.Intersection(excluded)
				visits <- VisitJob{v, neighbouring_candidates, neighbouring_excluded}
			}
			excluded.Add(v)
		}
		close(visits)
		wg.Wait()
		close(reporter.cliques)
	}()
	return gather_cliques(cliques)
}

type VisitJob struct {
	start      Vertex
	candidates VertexSet
	excluded   VertexSet
}
