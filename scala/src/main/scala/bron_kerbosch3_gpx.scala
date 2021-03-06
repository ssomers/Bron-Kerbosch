// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from both candidates and excluded vertices (IK_GPX)

import base.Vertex
import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocalX
import bron_kerbosch_pivot.visit

import scala.collection.immutable

object bron_kerbosch3_gpx extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Clique => Unit): Unit = {
    var excluded = Set.empty[Vertex]
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_candidates = neighbours &~ excluded
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(neighbours, excluded)
        visit(
          graph,
          reporter,
          MaxDegreeLocalX,
          MaxDegreeLocalX,
          neighbouring_candidates,
          neighbouring_excluded,
          immutable.List(v)
        )
      } else {
        assert(!util.are_disjoint(neighbours, excluded))
      }
      excluded += v
    }
  }
}
