import bron_kerbosch_pivot.PivotChoice.Arbitrary
import bron_kerbosch_pivot.visit

object bron_kerbosch2 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    val candidates = graph.connected_vertices().toSet
    if (candidates.nonEmpty) {
      visit(graph, Arbitrary, Arbitrary, candidates, Set(), Seq())
    } else {
      Seq()
    }
  }
}
