import base.Vertex

import scala.collection.immutable.TreeSet
import scala.collection.mutable

object RandomGraphGenerator {
  def random_choice(vseq: mutable.IndexedSeq[Vertex]): Vertex = {
    val i = util.Random.nextInt(vseq.size)
    vseq(i)
  }

  def random_sample(vset: mutable.Set[Vertex]): Vertex = {
    val i = util.Random.nextInt(vset.size)
    vset.iterator.drop(i).next()
  }

  def remove_from(vseq: mutable.IndexedSeq[Vertex], v: Vertex) = {
    val i = vseq.indexOf(v)
    val last = vseq.size - 1
    vseq(i) = vseq(last)
    vseq.patch(last, List(), 1)
  }

  def new_adjacencies(n: Int): mutable.IndexedSeq[mutable.Set[Vertex]] = {
    mutable.IndexedSeq.fill(n) { mutable.Set() }
  }

  def new_undirected(order: Int, size: Int): UndirectedGraph = {
    val fully_meshed_size = order
      .asInstanceOf[Long] * (order.asInstanceOf[Long] - 1) / 2
    require(
      size <= fully_meshed_size,
      f"$order nodes accommodate at most $fully_meshed_size edges"
    )
    var unsaturated_vertices: mutable.IndexedSeq[Vertex] =
      mutable.IndexedSeq(0 until order: _*)
    var adjacency_sets = new_adjacencies(order)
    var adjacency_complements = new_adjacencies(order)
    for (_ <- 1 to size) {
      var v: Vertex = -1
      var w: Vertex = -1
      v = random_choice(unsaturated_vertices)
      require(adjacency_sets(v).size < (order - 1))
      if (!adjacency_complements(v).isEmpty) {
        w = random_sample(adjacency_complements(v))
      } else {
        w = v
        while (w == v || adjacency_sets(v).contains(w)) {
          w = random_choice(unsaturated_vertices)
        }
      }
      assert(v != w)
      assert(!adjacency_sets(v).contains(w))
      assert(!adjacency_sets(w).contains(v))
      for ((x, y) <- List((v, w), (w, v))) {
        adjacency_sets(x) = adjacency_sets(x) + y
        val neighbours = adjacency_sets(x).size
        if (neighbours == order - 1) {
          remove_from(unsaturated_vertices, v)
        } else if (neighbours == order / 2) {
          // start using adjacency complement
          require(adjacency_complements(x).isEmpty)
          var s: mutable.Set[Vertex] = mutable.Set.empty
          s ++= unsaturated_vertices
          s --= adjacency_sets(x)
          s -= x
          adjacency_complements(x) = s
        } else if (neighbours > order / 2) {
          adjacency_complements(x).remove(y)
        }
      }
    }
    val adjacencies: List[Set[Vertex]] = List.empty ++ adjacency_sets
      .map(neighbours => new TreeSet[Vertex] ++ neighbours)
    val g = new SlimUndirectedGraph(adjacencies)
    require(g.order == order)
    require(g.size == size)
    g
  }
}
