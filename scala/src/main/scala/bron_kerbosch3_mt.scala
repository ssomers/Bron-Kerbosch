// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP), multi-threaded

import base.Vertex
import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocal
import bron_kerbosch_pivot.visit

import scala.collection.immutable
import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.duration.Duration
import scala.concurrent.{Await, Future}

object bron_kerbosch3_mt extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    Await.result(go_explore(graph), Duration.Inf)
  }

  def go_explore(graph: UndirectedGraph): Future[Cliques] = {
    var excluded = Set.empty[Vertex]
    var futures = List[Future[immutable.List[immutable.List[Vertex]]]]()
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_candidates = neighbours &~ excluded
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(neighbours, excluded)
        val f = Future {
          visit(
            graph,
            MaxDegreeLocal,
            MaxDegreeLocal,
            neighbouring_candidates,
            neighbouring_excluded,
            immutable.List(v)
          )
        }
        futures = f :: futures
      } else {
        assert(!util.are_disjoint(neighbours, excluded))
      }
      excluded += v
    }
    Future.foldLeft(futures)(immutable.List.empty[immutable.List[Vertex]]) {
      (collected, more) =>
        more ::: collected
    }
  }
}