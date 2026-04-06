package BronKerbosch

// Bron-Kerbosch algorithm with degeneracy ordering, multi-threaded

import "sync"

func bronKerbosch3gp0(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 1)
}

func bronKerbosch3gp1(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 4)
}

func bronKerbosch3gp2(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 16)
}

func bronKerbosch3gp3(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 64)
}

func bronKerbosch3gp4(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 256)
}

func bronKerbosch3om(graph *UndirectedGraph, consumer Consumer, numVisitors int) {
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
						graph, consumer,
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
			neighbouringCandidates := graph.neighbours(v).Difference(neighbouringExcluded)
			visits <- VisitJob{v, neighbouringCandidates, neighbouringExcluded}
		}
		close(visits)
		wg.Wait()
		consumer.close()
	}()
}

type VisitJob struct {
	start      Vertex
	candidates VertexSet
	excluded   VertexSet
}
