// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP)

import base.Vertex
import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocal
import bron_kerbosch_pivot.visit

import scala.collection.immutable

object bron_kerbosch3_gp extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    var cliques = immutable.List[Clique]()
    var excluded = Set.empty[Vertex]
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_candidates = neighbours &~ excluded
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(neighbours, excluded)
        cliques = visit(
          graph,
          MaxDegreeLocal,
          MaxDegreeLocal,
          neighbouring_candidates,
          neighbouring_excluded,
          immutable.List(v)
        ) ::: cliques
      } else {
        assert(!util.are_disjoint(neighbours, excluded))
      }
      excluded += v
    }
    cliques
  }
}
