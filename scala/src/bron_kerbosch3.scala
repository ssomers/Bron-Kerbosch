// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot arbitrarily

import base.Vertex
import bron_kerbosch_pivot.PivotChoice.Arbitrary
import bron_kerbosch_pivot.visit

object bron_kerbosch3 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Reporter): Unit = {
    var mut_excluded = Set.empty[Vertex]
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_candidates = neighbours &~ mut_excluded
      if (neighbouring_candidates.isEmpty) {
        assert(!util.is_disjoint(neighbours, mut_excluded))
      } else {
        val neighbouring_excluded = util.intersect(neighbours, mut_excluded)
        visit(
          graph,
          reporter,
          Arbitrary,
          Arbitrary,
          neighbouring_candidates,
          neighbouring_excluded,
          Seq(v)
        )
      }
      mut_excluded += v
    }
  }
}
