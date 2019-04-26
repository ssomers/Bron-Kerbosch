import base.Vertex

object bron_kerbosch1 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Reporter): Unit = {
    val candidates = graph.connected_vertices().toSet
    if (candidates.nonEmpty) {
      visit(graph, reporter, candidates, Set.empty, Seq.empty)
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

    while (true) {
      val v = candidates.head
      candidates = candidates.tail
      val neighbours = graph.neighbours(v)
      val neighbouring_candidates = util.intersect(neighbours, candidates)
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(neighbours, excluded)
        visit(
          graph,
          reporter,
          neighbouring_candidates,
          neighbouring_excluded,
          clique :+ v
        )
      } else {
        if (util.is_disjoint(neighbours, excluded)) {
          reporter.record(clique :+ v)
        }
        if (candidates.isEmpty) {
          return
        }
      }
      excluded += v
    }
  }
}
