package bron_kerbosch

import "sync"

func bron_kerbosch3_gp_2(graph *UndirectedGraph) [][]Vertex {
	return bron_kerbosch3om(graph, 5)
}

func bron_kerbosch3_gp_3(graph *UndirectedGraph) [][]Vertex {
	return bron_kerbosch3om(graph, 15)
}

func bron_kerbosch3_gp_4(graph *UndirectedGraph) [][]Vertex {
	return bron_kerbosch3om(graph, 45)
}

func bron_kerbosch3_gp_5(graph *UndirectedGraph) [][]Vertex {
	return bron_kerbosch3om(graph, 135)
}

func bron_kerbosch3om(graph *UndirectedGraph, num_visitors int) [][]Vertex {
	// Bron-Kerbosch algorithm with degeneracy ordering, multi-threaded

	starts := make(chan Vertex, num_visitors)
	visits := make(chan VisitJob, num_visitors)
	cliques := make(chan []Vertex)
	go degeneracy_ordering(graph, &ChannelVertexVisitor{starts}, -1)
	go func() {
		excluded := make(VertexSet, graph.connected_vertex_count()-1)
		reporter := ChannelReporter{cliques}
		var wg sync.WaitGroup
		wg.Add(num_visitors)
		for i := 0; i < num_visitors; i++ {
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
