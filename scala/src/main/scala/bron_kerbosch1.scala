import scala.collection.{immutable, mutable}

class bron_kerbosch1[Vertex] extends bron_kerbosch_algorithm[Vertex] {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): Unit = {
    val candidates = graph.connected_vertices().to(mutable.Set)
    if (candidates.nonEmpty)
      visit(graph, reporter, candidates, mutable.Set(), immutable.List())
  }

  def visit(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit,
      candidates: mutable.Set[Vertex],
      excluded: mutable.Set[Vertex],
      clique_in_progress: immutable.List[Vertex]
  ): Unit = {
    assert(candidates.forall(v => graph.degree(v) > 0))
    assert(excluded.forall(v => graph.degree(v) > 0))
    assert(util.are_disjoint(candidates, excluded))
    assert(candidates.nonEmpty)
    while (candidates.nonEmpty) {
      val v = candidates.head
      candidates -= v
      val neighbours = graph.neighbours(v)
      val neighbouring_candidates = util.intersect(neighbours, candidates)
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(neighbours, excluded)
        visit(
          graph,
          reporter,
          neighbouring_candidates.to(mutable.Set),
          neighbouring_excluded.to(mutable.Set),
          v :: clique_in_progress
        )
      } else if (util.are_disjoint(neighbours, excluded)) {
        reporter(v :: clique_in_progress)
      }
      excluded += v
    }
  }
}
