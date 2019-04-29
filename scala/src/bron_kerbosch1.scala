import base.Vertex

import scala.collection.mutable

object bron_kerbosch1 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    val candidates = graph.connected_vertices().toSet
    visit(graph, candidates, Set(), Seq())
  }

  def visit(graph: UndirectedGraph,
            initial_candidates: Set[Vertex],
            initial_excluded: Set[Vertex],
            clique_prefix: Seq[Vertex]): Cliques = {
    var cliques = new mutable.ArrayBuffer[Clique]()
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
        cliques ++= visit(
          graph,
          neighbouring_candidates,
          neighbouring_excluded,
          clique_prefix :+ v
        )
      } else {
        if (util.is_disjoint(neighbours, excluded)) {
          cliques += (clique_prefix :+ v)
        }
      }
      excluded += v
    }
    cliques
  }
}
