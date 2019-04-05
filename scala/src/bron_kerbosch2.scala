import base.Vertex

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
            clique: Seq[Vertex]): Unit = {
    var candidates = initial_candidates
    var excluded = initial_excluded
    assert(candidates.nonEmpty)
    assert(candidates.forall(v => graph.degree(v) > 0))
    assert(excluded.forall(v => graph.degree(v) > 0))

    val pivot = candidates.head
    val far_candidates = candidates diff graph.neighbours(pivot)
    for (v <- far_candidates) {
      val neighbours = graph.neighbours(v)
      candidates -= v
      val neighbouring_candidates = util.intersect(candidates, neighbours)
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(excluded, neighbours);
        visit(
          graph,
          reporter,
          neighbouring_candidates,
          neighbouring_excluded,
          clique :+ v
        )
      } else {
        if (util.is_disjoint(excluded, neighbours)) {
          reporter.record(clique :+ v)
        }
      }
      excluded += v
    }
  }
}
