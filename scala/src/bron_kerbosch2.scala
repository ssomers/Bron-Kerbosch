import base.{Clique, Vertex}

import scala.collection.mutable

object bron_kerbosch2 extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Reporter): Unit = {
    val candidates: mutable.Set[Vertex] = graph.connected_nodes()
    if (candidates.nonEmpty) {
      visit(graph, reporter, candidates, mutable.Set.empty[Vertex], List())
    }
  }

  def visit(graph: UndirectedGraph,
            reporter: Reporter,
            candidates: mutable.Set[Vertex],
            excluded: mutable.Set[Vertex],
            clique: Clique): Unit = {
    assert(candidates.forall(v => graph.degree(v) > 0))
    assert(excluded.forall(v => graph.degree(v) > 0))
    if (candidates.isEmpty) {
      if (excluded.isEmpty) {
        reporter.record(clique)
      }
      return
    }

    val pivot = candidates.head
    val far_candidates = candidates diff graph.neighbours(pivot)
    for (v <- far_candidates) {
      val neighbours = graph.neighbours(v)
      visit(
        graph,
        reporter,
        candidates & neighbours,
        excluded & neighbours,
        clique :+ v
      )
      candidates -= v
      excluded += v
    }
  }
}
