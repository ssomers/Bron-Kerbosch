// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from both candidates and excluded vertices (IK_GPX)

import base.Vertex
import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocalX
import bron_kerbosch_pivot.visit

import scala.collection.mutable

object bron_kerbosch3_gpx extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    var cliques = new mutable.ArrayBuffer[Clique]()
    var mut_excluded = Set.empty[Vertex]
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_candidates = neighbours &~ mut_excluded
      if (neighbouring_candidates.isEmpty) {
        assert(!util.is_disjoint(neighbours, mut_excluded))
      } else {
        val neighbouring_excluded = util.intersect(neighbours, mut_excluded)
        cliques ++= visit(
          graph,
          MaxDegreeLocalX,
          MaxDegreeLocalX,
          neighbouring_candidates,
          neighbouring_excluded,
          Seq(v)
        )
      }
      mut_excluded += v
    }
    cliques
  }
}
