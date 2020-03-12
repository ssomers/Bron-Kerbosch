import bron_kerbosch_pivot.PivotChoice.Arbitrary
import bron_kerbosch_pivot.visit

import scala.collection.immutable

object bron_kerbosch2 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Clique => Unit): Unit = {
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
