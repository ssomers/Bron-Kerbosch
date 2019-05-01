// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot arbitrarily

import base.Vertex
import bron_kerbosch_pivot.PivotChoice.Arbitrary
import bron_kerbosch_pivot.visit

import scala.collection.mutable

object bron_kerbosch3 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    var cliques = new mutable.ArrayBuffer[Clique]()
    var excluded = Set.empty[Vertex]
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_candidates = neighbours &~ excluded
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(neighbours, excluded)
        cliques ++= visit(
          graph,
          Arbitrary,
          Arbitrary,
          neighbouring_candidates,
          neighbouring_excluded,
          Seq(v)
        )
      } else {
        assert(!util.is_disjoint(neighbours, excluded))
      }
      excluded += v
    }
    cliques
  }
}
