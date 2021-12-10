// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP)

import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocal
import bron_kerbosch_pivot.visit

import scala.collection.{immutable, mutable}

class bron_kerbosch3_gp[Vertex: Integral]
    extends bron_kerbosch_algorithm[Vertex] {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): Unit = {
    // In this initial iteration, we don't need to represent the set of candidates
    // because all neighbours are candidates until excluded.
    var excluded = Set.empty[Vertex]
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_excluded = util.intersect(neighbours, excluded)
      if (neighbouring_excluded.size < neighbours.size) {
        val neighbouring_candidates = neighbours diff neighbouring_excluded
        assert(neighbouring_candidates.nonEmpty)
        visit(
          graph,
          reporter,
          MaxDegreeLocal,
          neighbouring_candidates.to(mutable.Set),
          neighbouring_excluded.to(mutable.Set),
          immutable.List(v)
        )
      }
      excluded += v
    }
  }
}
