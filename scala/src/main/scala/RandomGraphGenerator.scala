import base.Vertex

import scala.collection.immutable.HashSet
import scala.collection.mutable
import scala.util.Random

object RandomGraphGenerator {
  def random_choice(rng: Random, vseq: mutable.ArrayBuffer[Vertex]): Vertex = {
    val i = rng.nextInt(vseq.size)
    vseq(i)
  }

  def random_sample(rng: Random, vset: mutable.Set[Vertex]): Vertex = {
    val i = rng.nextInt(vset.size)
    vset.iterator.drop(i).next()
  }

  def remove_from(vseq: mutable.Buffer[Vertex], v: Vertex): Unit = {
    val i = vseq.indexOf(v)
    vseq(i) = vseq.last
    vseq.trimEnd(1)
  }

  def new_adjacencies(n: Int): mutable.IndexedSeq[mutable.Set[Vertex]] = {
    mutable.IndexedSeq.fill(n) { mutable.Set() }
  }

  def new_undirected(rng: Random, order: Int, size: Int): UndirectedGraph = {
    require(order > 0)
    val fully_meshed_size = order
      .asInstanceOf[Long] * (order.asInstanceOf[Long] - 1) / 2
    require(
      size <= fully_meshed_size,
      f"$order nodes accommodate at most $fully_meshed_size edges"
    )
    val unsaturated_vertices: mutable.ArrayBuffer[Vertex] =
      mutable.ArrayBuffer(0 until order: _*)
    val adjacency_sets = new_adjacencies(order)
    val adjacency_complements = new_adjacencies(order)
    for (_ <- 1 to size) {
      assert(
        unsaturated_vertices
          .forall(v => adjacency_sets(v).size < order - 1),
        adjacency_sets
      )
      val v = random_choice(rng, unsaturated_vertices)
      val w =
        if (adjacency_complements(v).nonEmpty) {
          random_sample(rng, adjacency_complements(v))
        } else {
          var w = v
          while (w == v || adjacency_sets(v).contains(w)) {
            w = random_choice(rng, unsaturated_vertices)
          }
          w
        }
      assert(v != w)
      assert(!adjacency_sets(v).contains(w))
      assert(!adjacency_sets(w).contains(v))
      for ((x, y) <- Array((v, w), (w, v))) {
        adjacency_sets(x) += y
        val neighbours = adjacency_sets(x).size
        if (neighbours == order - 1) {
          remove_from(unsaturated_vertices, x)
          adjacency_complements(x).clear()
        } else if (neighbours == order / 2) {
          // start using adjacency complement
          require(adjacency_complements(x).isEmpty)
          adjacency_complements(x) ++= unsaturated_vertices.filter(_ != x)
          adjacency_complements(x) --= adjacency_sets(x)
          assert(!adjacency_complements(x).contains(x))
        } else if (neighbours > order / 2) {
          val ok = adjacency_complements(x).remove(y)
          require(ok)
        }
      }
    }
    val adjacencies = IndexedSeq.empty[Set[Vertex]] ++ adjacency_sets
      .map(neighbours => new HashSet[Vertex] ++ neighbours)
    val g = new SlimUndirectedGraph(adjacencies)
    require(g.order == order)
    require(g.size == size)
    g
  }
}
