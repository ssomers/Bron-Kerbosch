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
      pivot_choice: PivotChoice,
      candidates: mutable.Set[Vertex],
      excluded: mutable.Set[Vertex],
      clique_in_progress: immutable.List[Vertex]
  ): Unit = {
    assert(candidates.forall(v => graph.degree(v) > 0))
    assert(excluded.forall(v => graph.degree(v) > 0))
    assert(util.are_disjoint(candidates, excluded))
    assert(candidates.nonEmpty)
    if (candidates.size == 1) {
      // Same logic as below, stripped down for this common case
      val v = candidates.head
      if (util.are_disjoint(graph.neighbours(v), excluded)) {
        reporter(v :: clique_in_progress)
      }
    } else {
      // Quickly handle locally unconnected candidates while finding pivot
      var pivot_ = None: Option[Vertex]
      val remaining_candidates =
        new mutable.ArrayBuffer[Vertex](candidates.size)
      var seen_local_degree = 0
      for (v <- candidates) {
        val neighbours = graph.neighbours(v)
        val local_degree = util.intersection_size(neighbours, candidates)
        if (local_degree == 0) {
          // Same logic as below, stripped down
          if (util.are_disjoint(neighbours, excluded)) {
            reporter(v :: clique_in_progress)
          }
        } else {
          if (seen_local_degree < local_degree) {
            seen_local_degree = local_degree
            pivot_ = Some(v)
          }
          remaining_candidates += v
        }
      }
      if (seen_local_degree == 0) {
        return
      }
      if (pivot_choice == MaxDegreeLocalX) {
        for (v <- excluded) {
          val neighbours = graph.neighbours(v)
          val local_degree = util.intersection_size(neighbours, candidates)
          if (seen_local_degree < local_degree) {
            seen_local_degree = local_degree
            pivot_ = Some(v)
          }
        }
      }
      val pivot = pivot_.get
      assert(remaining_candidates.nonEmpty)
      for (v <- remaining_candidates) {
        val neighbours = graph.neighbours(v)
        if (!neighbours.contains(pivot)) {
          candidates -= v
          val neighbouring_candidates = util.intersect(neighbours, candidates)
          if (neighbouring_candidates.nonEmpty) {
            val neighbouring_excluded = util.intersect(neighbours, excluded)
            visit(
              graph,
              reporter,
              pivot_choice,
              neighbouring_candidates.to(mutable.Set),
              neighbouring_excluded.to(mutable.Set),
              v :: clique_in_progress
            )
          } else if (util.are_disjoint(neighbours, excluded)) {
            reporter(v :: clique_in_progress)
          }
          excluded += v
        }
      }
    }
  }
}
