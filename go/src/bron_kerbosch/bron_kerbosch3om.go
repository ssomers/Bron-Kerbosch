package bron_kerbosch

import "sync"

func bron_kerbosch3om(graph *UndirectedGraph) [][]Vertex {
	// Bron-Kerbosch algorithm with degeneracy ordering, multi-threaded
	cliques := make(chan []Vertex)
	candidates := graph.connected_vertices()
	go bron_kerbosch3om_visit(graph, ChannelReporter{cliques}, &candidates)
	return gather_cliques(cliques)
}

type VisitJob struct {
	start      Vertex
	candidates *VertexSet
	excluded   *VertexSet
}

func bron_kerbosch3om_visit(graph *UndirectedGraph, reporter ChannelReporter,
	candidates *VertexSet) {
	excluded := make(VertexSet, len(*candidates))
	const NUM_VISITORTS = 8

	vertices := make(chan Vertex, NUM_VISITORTS)
	go degeneracy_ordering(graph, &ChannelVertexVisitor{vertices})
	visits := make(chan VisitJob, NUM_VISITORTS)
	var wg sync.WaitGroup
	wg.Add(NUM_VISITORTS)
	for i := 0; i < NUM_VISITORTS; i++ {
		go func() {
			for job := range visits {
				visit_max_degree(
					graph, &reporter,
					job.candidates,
					job.excluded,
					[]Vertex{job.start})
			}
			wg.Done()
		}()

	}
	for v := range vertices {
		neighbours := &graph.adjacencies[v]
		candidates.Remove(v)
		neighbouring_candidates := candidates.Intersection(neighbours)
		if !neighbouring_candidates.IsEmpty() {
			neighbouring_excluded := excluded.Intersection(neighbours)
			visits <- VisitJob{v,
				&neighbouring_candidates,
				&neighbouring_excluded}
		}
		excluded.Add(v)
	}
	close(visits)
	wg.Wait()
	close(reporter.cliques)
}
