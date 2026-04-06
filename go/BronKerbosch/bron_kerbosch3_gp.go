package BronKerbosch

// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP).

type BronKerbosch3gpDegeneracyVisitor struct {
	graph    *UndirectedGraph
	consumer Consumer
}

func (d *BronKerbosch3gpDegeneracyVisitor) visit(i DegeneracyVisitItem) {
	v := i.pick
	neighbouringExcluded := i.pickedNeighbours
	neighbouringCandidates := d.graph.neighbours(v).Difference(neighbouringExcluded)
	visit(
		d.graph, d.consumer,
		MaxDegreeLocal,
		neighbouringCandidates,
		neighbouringExcluded,
		[]Vertex{v})
}

func (d *BronKerbosch3gpDegeneracyVisitor) Close() {

}

func bronKerbosch3gp(graph *UndirectedGraph, consumer Consumer) {
	// In this initial iteration, we don't need to represent the set of candidates
	// because all neighbours are candidates until excluded.
	var ordering BronKerbosch3gpDegeneracyVisitor
	ordering.graph = graph
	ordering.consumer = consumer
	degeneracyVisitor(graph, &ordering)
	consumer.close()
}
