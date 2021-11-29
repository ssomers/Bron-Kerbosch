import scala.collection.{immutable, mutable}

object bron_kerbosch_pivot {
  object PivotChoice extends Enumeration {
    type PivotChoice = Value
    val Arbitrary, MaxDegree, MaxDegreeLocal, MaxDegreeLocalX = Value
  }
  import PivotChoice._

  def visit[Vertex](
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit,
      initial_pivot_choice: PivotChoice,
      further_pivot_choice: PivotChoice,
      initial_candidates: Set[Vertex],
      initial_excluded: Set[Vertex],
      clique_in_progress: immutable.List[Vertex]
  ): Unit = {
    var candidates = initial_candidates
    var excluded = initial_excluded
    assert(candidates.forall(v => graph.degree(v) > 0))
    assert(excluded.forall(v => graph.degree(v) > 0))
    assert(util.are_disjoint(candidates, excluded))
    assert(candidates.nonEmpty)
    if (candidates.size == 1) {
      // Same logic as below, stripped down for this common case
      val v = candidates.head
      if (util.are_disjoint(excluded, graph.neighbours(v))) {
        reporter(v :: clique_in_progress)
      }
    } else {
      val (remaining_candidates, pivot): (Iterable[Vertex], Vertex) =
        initial_pivot_choice match {
          case Arbitrary => (candidates, candidates.head)
          case MaxDegree => (candidates, candidates.maxBy(graph.degree))
          case MaxDegreeLocal |
              MaxDegreeLocalX => // Quickly handle locally unconnected candidates while finding pivot
            var pivot: Option[Vertex] = None
            val remaining_candidates =
              new mutable.ArrayBuffer[Vertex](candidates.size)
            var seen_local_degree = 0
            for (v <- candidates) {
              val neighbours = graph.neighbours(v)
              val local_degree = util.intersection_size(candidates, neighbours)
              if (local_degree == 0) {
                // Same logic as below, stripped down
                if (util.are_disjoint(excluded, neighbours)) {
                  reporter(v :: clique_in_progress)
                }
              } else {
                if (seen_local_degree < local_degree) {
                  seen_local_degree = local_degree
                  pivot = Some(v)
                }
                remaining_candidates += v
              }
            }
            if (seen_local_degree == 0) {
              return
            }
            if (initial_pivot_choice == MaxDegreeLocalX) {
              for (v <- excluded) {
                val neighbours = graph.neighbours(v)
                val local_degree =
                  util.intersection_size(candidates, neighbours)
                if (seen_local_degree < local_degree) {
                  seen_local_degree = local_degree
                  pivot = Some(v)
                }
              }
            }
            (remaining_candidates, pivot.get)
        }
      assert(remaining_candidates.nonEmpty)
      for (v <- remaining_candidates) {
        val neighbours = graph.neighbours(v)
        if (!neighbours.contains(pivot)) {
          candidates -= v
          val neighbouring_candidates = util.intersect(candidates, neighbours)
          if (neighbouring_candidates.nonEmpty) {
            val neighbouring_excluded = util.intersect(excluded, neighbours)
            visit(
              graph,
              reporter,
              further_pivot_choice,
              further_pivot_choice,
              neighbouring_candidates,
              neighbouring_excluded,
              v :: clique_in_progress
            )
          } else if (util.are_disjoint(excluded, neighbours)) {
            reporter(v :: clique_in_progress)
          }
          excluded += v
        }
      }
    }
  }
}
