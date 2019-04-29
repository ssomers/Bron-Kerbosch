import bron_kerbosch_pivot.PivotChoice.{MaxDegree, MaxDegreeLocalX}
import bron_kerbosch_pivot.visit

object bron_kerbosch2_gpx extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    val candidates = graph.connected_vertices().toSet
    if (candidates.nonEmpty) {
      visit(graph, MaxDegree, MaxDegreeLocalX, candidates, Set(), Seq())
    } else {
      Seq()
    }
  }
}
