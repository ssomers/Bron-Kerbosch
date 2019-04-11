import base.Vertex

object bron_kerbosch2 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Reporter): Unit = {
    val candidates: Set[Vertex] = graph.connected_vertices()
    if (candidates.nonEmpty) {
      visit(graph, reporter, candidates, Set.empty[Vertex], List())
    }
  }

  def visit(graph: UndirectedGraph,
            reporter: Reporter,
            initial_candidates: Set[Vertex],
            initial_excluded: Set[Vertex],
            clique: Seq[Vertex]): Unit = {
    assert(initial_candidates.nonEmpty)
    assert(initial_candidates.forall(v => graph.degree(v) > 0))
    assert(initial_excluded.forall(v => graph.degree(v) > 0))

    if (initial_candidates.size == 1) {
      for (v <- initial_candidates) {
        val neighbours = graph.neighbours(v)
        if (util.is_disjoint(initial_excluded, neighbours)) {
          reporter.record(clique :+ v)
        }
      }
      return
    }

    var candidates = initial_candidates
    var excluded = initial_excluded
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
