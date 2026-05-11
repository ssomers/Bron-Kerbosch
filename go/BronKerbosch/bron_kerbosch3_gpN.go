package BronKerbosch

// Bron-Kerbosch algorithm with degeneracy ordering, multi-threaded

import "sync"

func bronKerbosch3gp1(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 1)
}

func bronKerbosch3gp2(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 2)
}

func bronKerbosch3gp3(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 3)
}

func bronKerbosch3gp4(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 4)
}

func bronKerbosch3gp5(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 5)
}

func bronKerbosch3gp6(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 6)
}

func bronKerbosch3gp8(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 8)
}

func bronKerbosch3gp24(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 24)
}

func bronKerbosch3gp72(graph *UndirectedGraph, consumer Consumer) {
	bronKerbosch3om(graph, consumer, 72)
}

func bronKerbosch3om(graph *UndirectedGraph, consumer Consumer, numVisitors int) {
	visits := make(chan VisitJob, numVisitors)
	go func() {
		degeneracyVisitor(graph, func(i DegeneracyVisitItem) {
			v := i.pick
			neighbouringCandidates, neighbouringExcluded :=
				graph.neighbours(v).Partition(i.isCandidate)
			visits <- VisitJob{v, neighbouringCandidates, neighbouringExcluded}
		})
		close(visits)
	}()
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
	wg.Wait()
}

type VisitJob struct {
	start      Vertex
	candidates VertexSet
	excluded   VertexSet
}
