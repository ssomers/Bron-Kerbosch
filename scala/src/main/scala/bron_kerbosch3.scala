// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot arbitrarily

import bron_kerbosch_pivot.PivotChoice.Arbitrary
import bron_kerbosch_pivot.visit

import scala.collection.immutable

class bron_kerbosch3[Vertex: Integral] extends bron_kerbosch_algorithm[Vertex] {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): Unit = {
    var excluded = Set.empty[Vertex]
    for (v <- degeneracy_ordering[Vertex](graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_candidates = neighbours &~ excluded
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(neighbours, excluded)
        visit(
          graph,
          reporter,
          Arbitrary,
          Arbitrary,
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
