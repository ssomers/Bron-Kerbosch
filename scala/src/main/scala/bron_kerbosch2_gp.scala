import bron_kerbosch_pivot.PivotChoice.{MaxDegree, MaxDegreeLocal}
import bron_kerbosch_pivot.visit

import scala.collection.immutable

class bron_kerbosch2_gp[Vertex] extends bron_kerbosch_algorithm[Vertex] {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): Unit = {
    val candidates = graph.connected_vertices().toSet
    if (candidates.nonEmpty) {
      visit(
        graph,
        reporter,
        MaxDegree,
        MaxDegreeLocal,
        candidates,
        Set(),
        immutable.List()
      )
    }
  }
}
