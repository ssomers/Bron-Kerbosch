import base.{Clique, Vertex}

import scala.collection.immutable.{SortedSet, TreeSet}
import scala.util.Random

object main {
  val FUNC_NAMES = List("Ver1", "Ver2")
  val FUNCS = List(bron_kerbosch1, bron_kerbosch2)

  type OrderedClique = SortedSet[Vertex]
  type OrderedCliques = SortedSet[OrderedClique]
  implicit def CliqueOrdering: Ordering[OrderedClique] =
    (a: OrderedClique, b: OrderedClique) => {
      require(a.size > 1)
      require(b.size > 1)
      a.iterator
        .zip(b.iterator)
        .map { case (va, vb) => va - vb }
        .find { _ != 0 }
        .getOrElse(a.size - b.size)
    }
  def order_cliques(cliques: List[Clique]): OrderedCliques = {
    new TreeSet[OrderedClique] ++ cliques
      .map(clique => new TreeSet[Vertex] ++ clique.toSet)
      .toSet
  }

  def bron_kerbosch_timed(graph: UndirectedGraph,
                          samples: Int): Array[SampleStatistics] = {
    var first: Option[OrderedCliques] = None
    val times = Array.fill(FUNCS.size) { new SampleStatistics }

    for (sample <- 1 to samples; (func, func_index) <- FUNCS.zipWithIndex) {
      val reporter = new SimpleReporter
      val start = System.currentTimeMillis()

      func.explore(graph, reporter)
      val elapsed = System.currentTimeMillis() - start
      times(func_index).put(elapsed)

      if (samples > 1 && sample <= 2) {
        val orderedCliques = order_cliques(reporter.cliques.toList)
        first match {
          case None               => first = Some(orderedCliques)
          case Some(firstCliques) => require(firstCliques == orderedCliques)
        }
      }
    }
    times
  }

  def bk(order_str: String,
         order: Int,
         sizes: List[Int],
         samples: Int): Unit = {
    val name = "bron_kerbosch_scala_order_" + order_str
    val path = f"..\\$name.csv"

    val fo = new java.io.PrintWriter(new java.io.File(path))
    fo.print("Size")
    for (name <- FUNC_NAMES) {
      fo.print(f",$name min,$name mean,$name max")
    }
    fo.println()

    for (size <- sizes) {
      val rng = new Random(19680516L)
      val graph = RandomGraphGenerator.new_undirected(rng, order, size)
      val times = bron_kerbosch_timed(graph, samples)

      fo.print(f"$size")
      for ((func_name, func_index) <- FUNC_NAMES.zipWithIndex) {
        val max = times(func_index).max / 1e3
        val min = times(func_index).min / 1e3
        val mean = times(func_index).mean / 1e3
        val dev = times(func_index).deviation / 1e3
        fo.print(f",$min,$mean,$max")
        println(
          f"order $order%7d size $size%7d $func_name%8s: $mean%5.2f ±$dev%5.2f"
        )
      }
      fo.println()
    }
    fo.close()
  }

  def main(args: Array[String]) {
    //noinspection NameBooleanParameters
    assert(false, "Turn off assertions for meaningful measurements")

    val k = 1000
    val M = k * k
    val sizes_100: List[Int] = List((2 * k) to (3 * k) by 50: _*)
    val sizes_10k: List[Int] = List(
      ((1 * k) until (10 * k) by k) ++ ((10 * k) to (200 * k) by 10 * k): _*
    )
    val sizes_1M: List[Int] = List(
      (0 until (1 * M) by 250 * k) ++ ((1 * M) to (3 * M) by 500 * k): _*
    )
    Thread.sleep(4321) // give launcher some time to cool down
    bk("init", 2, List(1), 3)
      bk("100", 100, sizes_100, 5)
      bk("10k", 10 * k, sizes_10k, 5)
    bk("1M", 1 * M, sizes_1M, 3)
  }
}
