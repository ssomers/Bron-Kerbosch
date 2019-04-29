import base.Vertex

import scala.collection.mutable

object bron_kerbosch_pivot {
  object PivotChoice extends Enumeration {
    type PivotChoice = Value
    val Arbitrary, MaxDegree, MaxDegreeLocal, MaxDegreeLocalX = Value
  }
  import PivotChoice._

  def visit(graph: UndirectedGraph,
            initial_pivot_choice: PivotChoice,
            further_pivot_choice: PivotChoice,
            candidates: Set[Vertex],
            excluded: Set[Vertex],
            clique_prefix: Seq[Vertex]): bron_kerbosch_algorithm#Cliques = {
    assert(candidates.nonEmpty)
    assert(candidates.forall(v => graph.degree(v) > 0))
    assert(excluded.forall(v => graph.degree(v) > 0))
    assert(util.is_disjoint(candidates, excluded))

    if (candidates.size == 1) {
      // Same logic as below, stripped down for this common case
      val v = candidates.head
      val neighbours = graph.neighbours(v)
      if (util.is_disjoint(neighbours, excluded)) {
        Seq(clique_prefix :+ v)
      } else {
        Seq()
      }
    }

    val cliques = new mutable.ArrayBuffer[Seq[Vertex]]
    val (remaining_candidates, pivot): (Traversable[Vertex], Vertex) =
      initial_pivot_choice match {
        case Arbitrary => (candidates, candidates.head)
        case MaxDegree => (candidates, candidates.maxBy(graph.degree))
        case MaxDegreeLocal |
            MaxDegreeLocalX => // Quickly handle locally unconnected candidates while finding pivot
          var pivot: Option[Vertex] = None
          var remaining_candidates =
            new mutable.ArrayBuffer[Vertex](candidates.size)
          var seen_local_degree = 0
          for (v <- candidates) {
            val neighbours = graph.neighbours(v)
            val local_degree = util.intersection_size(neighbours, candidates)
            if (local_degree == 0) {
              // Same logic as below, stripped down
              if (util.is_disjoint(neighbours, excluded)) {
                cliques += clique_prefix :+ v
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
            return cliques
          }
          if (initial_pivot_choice == MaxDegreeLocalX) {
            for (v <- excluded) {
              val neighbours = graph.neighbours(v)
              val local_degree = util.intersection_size(neighbours, candidates)
              if (seen_local_degree < local_degree) {
                seen_local_degree = local_degree
                pivot = Some(v)
              }
            }
          }
          (remaining_candidates, pivot.get)
      }
    assert(remaining_candidates.nonEmpty)
    var mut_candidates = candidates
    var mut_excluded = excluded
    for (v <- remaining_candidates) {
      val neighbours = graph.neighbours(v)
      if (!neighbours.contains(pivot)) {
        mut_candidates -= v
        val neighbouring_candidates = util.intersect(neighbours, mut_candidates)
        if (neighbouring_candidates.isEmpty) {
          if (util.is_disjoint(neighbours, mut_excluded)) {
            cliques += (clique_prefix :+ v)
          }
        } else {
          val neighbouring_excluded = util.intersect(neighbours, mut_excluded)
          cliques ++= visit(
            graph,
            further_pivot_choice,
            further_pivot_choice,
            neighbouring_candidates,
            neighbouring_excluded,
            clique_prefix :+ v
          )
        }
        mut_excluded += v
      }
    }
    cliques
  }
}
