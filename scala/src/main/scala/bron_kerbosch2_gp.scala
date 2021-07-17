import base.Vertex
import bron_kerbosch_pivot.PivotChoice.{MaxDegree, MaxDegreeLocal}
import bron_kerbosch_pivot.visit

import scala.collection.immutable

object bron_kerbosch2_gp extends bron_kerbosch_algorithm {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: Clique => Unit
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
