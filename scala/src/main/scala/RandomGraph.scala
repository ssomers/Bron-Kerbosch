import scala.collection.immutable.HashSet
import scala.collection.mutable
import scala.io.Source
import scala.util.Random

object RandomGraph {
  def random_choice[Vertex](
      rng: Random,
      vseq: mutable.ArrayBuffer[Vertex]
  ): Vertex = {
    val i = rng.nextInt(vseq.size)
    vseq(i)
  }

  def random_sample[Vertex](rng: Random, vset: mutable.Set[Vertex]): Vertex = {
    val i = rng.nextInt(vset.size)
    vset.iterator.drop(i).next()
  }

  def remove_from[Vertex](vseq: mutable.Buffer[Vertex], v: Vertex): Unit = {
    val i = vseq.indexOf(v)
    vseq(i) = vseq.last
    vseq.dropRightInPlace(1)
  }

  private def new_adjacencies[Vertex](
      n: Int
  ): mutable.IndexedSeq[mutable.Set[Vertex]] = {
    mutable.IndexedSeq.fill(n) { mutable.Set() }
  }

  def read_undirected[Vertex: Integral](
      order_str: String,
      order: Int,
      size: Int
  ): (UndirectedGraph[Vertex], Int) = {
    require(order > 0)
    val fully_meshed_size = order
      .asInstanceOf[Long] * (order.asInstanceOf[Long] - 1) / 2
    require(
      size <= fully_meshed_size,
      f"$order nodes accommodate at most $fully_meshed_size edges"
    )
    val edges_path = f"..\\data\\random_edges_order_$order_str.txt"
    val stats_path = f"..\\data\\random_stats.txt"
    val adjacencies = read_edges[Vertex](edges_path, order, size)
    val clique_count = read_stats(stats_path, order_str, size)
    val graph = new SlimUndirectedGraph(adjacencies)
    require(graph.order() == order)
    require(graph.size() == size)
    (graph, clique_count)
  }

  private def read_edges[Vertex: Integral](
      path: String,
      order: Int,
      size: Int
  ): IndexedSeq[Set[Vertex]] = {
    val adjacency_sets = new_adjacencies[Vertex](order)
    val file = Source.fromFile(path)
    for (line <- file.getLines().take(size)) {
      val fields = line.split(" ", 2)
      val v = Integer.parseInt(fields(0))
      val w = Integer.parseInt(fields(1))
      adjacency_sets(v) += implicitly[Integral[Vertex]].fromInt(w)
      adjacency_sets(w) += implicitly[Integral[Vertex]].fromInt(v)
    }
    file.close()
    IndexedSeq.empty[Set[Vertex]] ++ adjacency_sets
      .map(neighbours => new HashSet[Vertex] ++ neighbours)
  }

  private def read_stats(path: String, order_str: String, size: Int): Int = {
    val prefix = f"$order_str\t$size\t"
    val file = Source.fromFile(path)
    var stats: String = ""
    for (line <- file.getLines().take(size)) {
      if (line.startsWith(prefix)) {
        stats = line.substring(prefix.length)
      }
    }
    file.close()
    if (stats.isEmpty)
      throw new IllegalArgumentException(
        f"File $path lacks order $order_str size $size"
      )
    Integer.parseInt(stats)
  }
}
