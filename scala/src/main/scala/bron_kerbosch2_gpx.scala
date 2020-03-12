import bron_kerbosch_pivot.PivotChoice.{MaxDegree, MaxDegreeLocalX}
import bron_kerbosch_pivot.visit

import scala.collection.immutable

object bron_kerbosch2_gpx extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Clique => Unit): Unit = {
    val candidates = graph.connected_vertices().toSet
    if (candidates.nonEmpty) {
      visit(
        graph,
        reporter,
        MaxDegree,
        MaxDegreeLocalX,
        candidates,
        Set(),
        immutable.List()
      )
    }
  }
}
