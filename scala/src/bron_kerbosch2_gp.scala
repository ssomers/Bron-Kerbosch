import base.Vertex
import bron_kerbosch_pivot.PivotChoice.{MaxDegree, MaxDegreeLocal}
import bron_kerbosch_pivot.visit

object bron_kerbosch2_gp extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Reporter): Unit = {
    val candidates = graph.connected_vertices().toSet
    if (candidates.nonEmpty) {
      visit(
        graph,
        reporter,
        MaxDegree,
        MaxDegreeLocal,
        candidates,
        Set.empty,
        Seq.empty
      )
    }
  }
}
