// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP)

import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocal
import bron_kerbosch_pivot.visit

import scala.collection.immutable

class bron_kerbosch3_gp[Vertex: Integral]
    extends bron_kerbosch_algorithm[Vertex] {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): Unit = {
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
          MaxDegreeLocal,
          MaxDegreeLocal,
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
