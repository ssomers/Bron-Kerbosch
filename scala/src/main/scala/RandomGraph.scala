import base.Vertex

import scala.collection.immutable.HashSet
import scala.collection.mutable
import scala.io.Source
import scala.util.Random

object RandomGraph {
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

  def read_undirected(order_str: String,
                      order: Int,
                      size: Int): UndirectedGraph = {
    require(order > 0)
    val fully_meshed_size = order
      .asInstanceOf[Long] * (order.asInstanceOf[Long] - 1) / 2
    require(
      size <= fully_meshed_size,
      f"$order nodes accommodate at most $fully_meshed_size edges"
    )
    val path = f"..\\random_edges_order_$order_str.txt"
    var linenum = 0;
    val adjacency_sets = new_adjacencies(order)
    for (line <- Source.fromFile(path).getLines.take(size)) {
      linenum += 1
      val fields = line.split(" ", 2)
      val v = Integer.parseInt(fields(0))
      val w = Integer.parseInt(fields(1))
      adjacency_sets(v) += w
      adjacency_sets(w) += v
    }
    val adjacencies = IndexedSeq.empty[Set[Vertex]] ++ adjacency_sets
      .map(neighbours => new HashSet[Vertex] ++ neighbours)
    val g = new SlimUndirectedGraph(adjacencies)
    require(g.order == order)
    require(g.size == size)
    g
  }
}
