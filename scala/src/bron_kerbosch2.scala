import base.{Clique, Vertex}

object bron_kerbosch2 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Reporter): Unit = {
    val candidates: Set[Vertex] = graph.connected_nodes()
    if (candidates.nonEmpty) {
      visit(graph, reporter, candidates, Set.empty[Vertex], List())
    }
  }

  def visit(graph: UndirectedGraph,
            reporter: Reporter,
            initial_candidates: Set[Vertex],
            initial_excluded: Set[Vertex],
            clique: Clique): Unit = {
    var candidates = initial_candidates
    var excluded = initial_excluded
    assert(candidates.forall(v => graph.degree(v) > 0))
    assert(excluded.forall(v => graph.degree(v) > 0))
    if (candidates.isEmpty) {
      if (excluded.isEmpty) {
        reporter.record(clique)
      }
      return
    }

    val pivot = candidates.head
    val far_candidates = candidates diff graph.neighbours(pivot)
    for (v <- far_candidates) {
      val neighbours = graph.neighbours(v)
      candidates -= v
      visit(
        graph,
        reporter,
        candidates & neighbours,
        excluded & neighbours,
        clique :+ v
      )
      excluded += v
    }
  }
}
