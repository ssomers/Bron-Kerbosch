import bron_kerbosch_pivot.PivotChoice.MaxDegree
import bron_kerbosch_pivot.visit

object bron_kerbosch2_g extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    val candidates = graph.connected_vertices().toSet
    if (candidates.nonEmpty) {
      visit(graph, MaxDegree, MaxDegree, candidates, Set(), Seq())
    } else {
      Seq()
    }
  }
}
