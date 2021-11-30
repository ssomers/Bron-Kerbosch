import bron_kerbosch_pivot.PivotChoice.Arbitrary
import bron_kerbosch_pivot.visit

import scala.collection.{immutable, mutable}

class bron_kerbosch2[Vertex] extends bron_kerbosch_algorithm[Vertex] {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): Unit = {
    val candidates = graph.connected_vertices().to(mutable.Set)
    if (candidates.nonEmpty) {
      visit(
        graph,
        reporter,
        Arbitrary,
        Arbitrary,
        candidates,
        mutable.Set(),
        immutable.List()
      )
    } else {
      immutable.Iterable()
    }
  }
}
