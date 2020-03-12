import base.Vertex

import scala.collection.immutable

object bron_kerbosch1 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Clique => Unit): Unit = {
    val candidates = graph.connected_vertices().toSet
    visit(graph, reporter, candidates, Set(), immutable.List())
  }

  def visit(graph: UndirectedGraph,
            reporter: Clique => Unit,
            initial_candidates: Set[Vertex],
            initial_excluded: Set[Vertex],
            clique_in_progress: immutable.List[Vertex]): Unit = {
    var candidates = initial_candidates
    var excluded = initial_excluded
    assert(candidates.forall(v => graph.degree(v) > 0))
    assert(excluded.forall(v => graph.degree(v) > 0))

    while (candidates.nonEmpty) {
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
          v :: clique_in_progress
        )
      } else {
        if (util.are_disjoint(neighbours, excluded)) {
          reporter(v :: clique_in_progress)
        }
      }
      excluded += v
    }
  }
}
