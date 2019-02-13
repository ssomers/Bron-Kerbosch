import base.{Clique, Vertex, intersect}

object bron_kerbosch1 extends bron_kerbosch_algorithm {
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
    if (candidates.isEmpty && excluded.isEmpty) {
      reporter.record(clique)
      return
    }

    while (candidates.nonEmpty) {
      val v = candidates.head
      candidates = candidates.tail
      val neighbours = graph.neighbours(v)
      visit(
        graph,
        reporter,
        intersect(candidates, neighbours),
        intersect(excluded, neighbours),
        clique :+ v
      )
      excluded += v
    }
  }
}
