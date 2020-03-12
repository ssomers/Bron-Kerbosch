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
                      size: Int): (UndirectedGraph, Int) = {
    require(order > 0)
    val fully_meshed_size = order
      .asInstanceOf[Long] * (order.asInstanceOf[Long] - 1) / 2
    require(
      size <= fully_meshed_size,
      f"$order nodes accommodate at most $fully_meshed_size edges"
    )
    val edges_path = f"..\\random_edges_order_$order_str.txt"
    val stats_path = f"..\\random_stats.txt"
    val adjacencies = read_edges(edges_path, order, size)
    val clique_count = read_stats(stats_path, order_str, size)
    val g = new SlimUndirectedGraph(adjacencies)
    require(g.order == order)
    require(g.size == size)
    (g, clique_count)
  }

  def read_edges(path: String,
                 order: Int,
                 size: Int): IndexedSeq[Set[Vertex]] = {
    val adjacency_sets = new_adjacencies(order)
    val file = Source.fromFile(path)
    for (line <- file.getLines.take(size)) {
      val fields = line.split(" ", 2)
      val v = Integer.parseInt(fields(0))
      val w = Integer.parseInt(fields(1))
      adjacency_sets(v) += w
      adjacency_sets(w) += v
    }
    file.close()
    IndexedSeq.empty[Set[Vertex]] ++ adjacency_sets
      .map(neighbours => new HashSet[Vertex] ++ neighbours)
  }

  def read_stats(path: String, order_str: String, size: Int): Int = {
    val prefix = f"$order_str\t$size\t"
    val file = Source.fromFile(path)
    for (line <- file.getLines.take(size)) {
      if (line.startsWith(prefix)) {
        val c = Integer.parseInt(line.substring(prefix.length))
        return c
      }
    }
    file.close()
    throw new IllegalArgumentException(
      f"File $path lacks order $order_str size $size"
    )
  }
}
