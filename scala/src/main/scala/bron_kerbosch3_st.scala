// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP), multi-threaded

import base.Vertex
import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocal
import bron_kerbosch_pivot.visit

import scala.collection.immutable
import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.duration.Duration
import scala.concurrent.{Await, Future}

object bron_kerbosch3_st extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph, reporter: Clique => Unit): Unit = {
    val futures = go_explore(graph, (clique: Clique) => {
      this.synchronized {
        reporter(clique)
      }
    })
    for (future <- futures)
      Await.ready(future, Duration.Inf)
  }

  def go_explore(graph: UndirectedGraph,
                 reporter: Clique => Unit): List[Future[Unit]] = {
    var excluded = Set.empty[Vertex]
    var futures = List[Future[Unit]]()
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_candidates = neighbours &~ excluded
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(neighbours, excluded)
        val f = Future {
          visit(
            graph,
            reporter,
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
    futures
  }
}
