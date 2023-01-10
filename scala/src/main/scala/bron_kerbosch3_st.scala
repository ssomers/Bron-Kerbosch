// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP), multi-threaded

import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocal
import bron_kerbosch_pivot.visit

import scala.collection.{immutable, mutable}
import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent.duration.Duration
import scala.concurrent.{Await, Future}

class bron_kerbosch3_st[Vertex: Integral]
    extends bron_kerbosch_algorithm[Vertex] {
  def explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): Unit = {
    val futures = go_explore(
      graph,
      (clique: immutable.Iterable[Vertex]) => {
        this.synchronized {
          reporter(clique)
        }
      }
    )
    for (future <- futures)
      Await.ready(future, Duration.Inf)
  }

  private def go_explore(
      graph: UndirectedGraph[Vertex],
      reporter: immutable.Iterable[Vertex] => Unit
  ): List[Future[Unit]] = {
    // In this initial iteration, we don't need to represent the set of candidates
    // because all neighbours are candidates until excluded.
    var excluded = Set.empty[Vertex]
    var futures = List[Future[Unit]]()
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_excluded = util.intersect(neighbours, excluded)
      if (neighbouring_excluded.size < neighbours.size) {
        val neighbouring_candidates = neighbours diff neighbouring_excluded
        val f = Future {
          visit(
            graph,
            reporter,
            MaxDegreeLocal,
            neighbouring_candidates.to(mutable.Set),
            neighbouring_excluded.to(mutable.Set),
            immutable.List(v)
          )
        }
        futures = f :: futures
      }
      excluded += v
    }
    futures
  }
}
