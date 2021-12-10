import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocal
import bron_kerbosch_pivot.visit

import scala.collection.{immutable, mutable}

class bron_kerbosch2_gp[Vertex: Integral]
    extends bron_kerbosch_algorithm[Vertex] {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): Unit = {
    val order = graph.order()
    if (order == 0) {
      return
    }
    val pivot = graph.max_degree_vertex()
    // In this initial iteration, we don't need to represent the set of candidates
    // because all neighbours are candidates until excluded.
    var excluded = Set.empty[Vertex]
    for (i <- 0 until order) {
      val v = implicitly[Integral[Vertex]].fromInt(i)
      val neighbours = graph.neighbours(v)
      if (neighbours.nonEmpty && !neighbours.contains(pivot)) {
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
}
