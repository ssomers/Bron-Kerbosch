import bron_kerbosch_pivot.PivotChoice.{MaxDegree, MaxDegreeLocal}
import bron_kerbosch_pivot.visit

object bron_kerbosch2_gp extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    val candidates = graph.connected_vertices().toSet
    if (candidates.nonEmpty) {
      visit(graph, MaxDegree, MaxDegreeLocal, candidates, Set(), Seq())
    } else {
      Iterable()
    }
  }
}
