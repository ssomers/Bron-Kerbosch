import bron_kerbosch_pivot.PivotChoice.Arbitrary
import bron_kerbosch_pivot.visit

import scala.collection.immutable

class bron_kerbosch2[Vertex] extends bron_kerbosch_algorithm[Vertex] {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): Unit = {
    val candidates = graph.connected_vertices().toSet
    if (candidates.nonEmpty) {
      visit(
        graph,
        reporter,
        Arbitrary,
        Arbitrary,
        candidates,
        Set(),
        immutable.List()
      )
    } else {
      immutable.Iterable()
    }
  }
}
