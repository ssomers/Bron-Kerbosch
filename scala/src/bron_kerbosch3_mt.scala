// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot from candidates only (IK_GP), multi-threaded

import base.Vertex
import bron_kerbosch_pivot.PivotChoice.MaxDegreeLocal
import bron_kerbosch_pivot.visit

import scala.collection.mutable
import scala.concurrent.ExecutionContext.Implicits.global
import scala.concurrent._
import scala.concurrent.duration._

object bron_kerbosch3_mt extends bron_kerbosch_algorithm {
  def explore(graph: UndirectedGraph): Cliques = {
    var mut_excluded = Set.empty[Vertex]
    var futures = new mutable.ListBuffer[Future[Cliques]]()
    for (v <- degeneracy_ordering(graph, -1)) {
      val neighbours = graph.neighbours(v)
      assert(neighbours.nonEmpty)
      val neighbouring_candidates = neighbours &~ mut_excluded
      if (neighbouring_candidates.nonEmpty) {
        val neighbouring_excluded = util.intersect(neighbours, mut_excluded)
        val f = Future {
          visit(
            graph,
            MaxDegreeLocal,
            MaxDegreeLocal,
            neighbouring_candidates,
            neighbouring_excluded,
            Seq(v)
          )
        }
        futures += f
      } else {
        assert(!util.is_disjoint(neighbours, mut_excluded))
      }
      mut_excluded += v
    }
    var cliques = new mutable.ArrayBuffer[Clique]()
    for (f <- futures)
      cliques ++= Await.result(f, Duration.Inf)
    cliques
  }
}
