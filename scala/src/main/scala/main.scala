import base.Vertex

import scala.collection.mutable.ArrayBuffer

object main {
  val FUNC_NAMES: IndexedSeq[String] =
    IndexedSeq(
      "Ver1+",
      "Ver2+",
      "Ver2+G",
      "Ver2+GP",
      "Ver2+GPX",
      "Ver3+",
      "Ver3+GP",
      "Ver3+GPX",
      "Ver3=ST"
    )
  val FUNCS: IndexedSeq[bron_kerbosch_algorithm] = IndexedSeq(
    bron_kerbosch1,
    bron_kerbosch2,
    bron_kerbosch2_g,
    bron_kerbosch2_gp,
    bron_kerbosch2_gpx,
    bron_kerbosch3,
    bron_kerbosch3_gp,
    bron_kerbosch3_gpx,
    bron_kerbosch3_st
  )

  type Clique = bron_kerbosch_algorithm#Clique
  type Cliques = ArrayBuffer[Clique]
  def order_cliques(cliques: Cliques): Seq[Seq[Vertex]] = {
    require(cliques.forall(_.size > 1))
    cliques
      .map(clique => clique.toSeq.sortWith(_.compareTo(_) < 0))
      .toSeq
      .sortWith((a: Seq[Vertex], b: Seq[Vertex]) => {
        val diff = a.iterator
          .zip(b.iterator)
          .map { case (va, vb) => va.compareTo(vb) }
          .find { _ != 0 }
        if (diff.isEmpty) {
          throw new IllegalArgumentException(
            f"got overlapping or equal cliques $a <> $b"
          )
        }
        diff.get < 0
      })
  }

  def bron_kerbosch_timed(
      graph: UndirectedGraph[Vertex],
      clique_count: Int,
      samples: Int,
      func_indices: Array[Int]
  ): Array[SampleStatistics] = {
    var firstOrdered: Option[Seq[Seq[Vertex]]] = None
    val times = Array.fill(FUNCS.size) { new SampleStatistics }
    val start = if (samples == 1) 1 else 0

    for (sample <- start to samples; func_index <- func_indices) {
      val func = FUNCS(func_index)
      val func_name = FUNC_NAMES(func_index)
      if (sample == 0) {
        val cliques = new Cliques()
        val reporter = (clique: Clique) => { cliques += clique; () }
        func.explore(graph, reporter)
        if (cliques.size != clique_count)
          throw new AssertionError(
            f"$func_name: Expected $clique_count, obtained ${cliques.size} cliques"
          )
        val ordered = order_cliques(cliques)
        firstOrdered match {
          case None           => firstOrdered = Some(ordered)
          case Some(ordered1) => require(ordered1 == ordered, func_name)
        }
      } else {
        var cliques = 0
        val reporter = (_: Clique) => { cliques += 1; () }
        val start = System.nanoTime()
        func.explore(graph, reporter)
        val elapsed = (System.nanoTime() - start) / 1e9
        times(func_index).put(elapsed)
      }
    }
    times
  }

  def bk(
      order_str: String,
      order: Int,
      sizes: Array[Int],
      samples: Int,
      func_indices: Array[Int]
  ): Unit = {
    val name = "bron_kerbosch_scala_order_" + order_str
    val path = f"..\\$name.csv"

    val fo = new java.io.PrintWriter(new java.io.File(path))
    fo.print("Size")
    for (func_index <- func_indices) {
      val name = FUNC_NAMES(func_index)
      fo.print(f",$name min,$name mean,$name max")
    }
    fo.println()

    for (size <- sizes) {
      val start = System.nanoTime()
      val (graph, clique_count) =
        RandomGraph.read_undirected[Vertex](order_str, order, size)
      val elapsed = (System.nanoTime() - start) / 1e9
      println(f"$order_str%7s nodes, $size%7d edges, creation: $elapsed%6.3f")
      val times =
        bron_kerbosch_timed(graph, clique_count, samples, func_indices)

      fo.print(f"$size")
      for (func_index <- func_indices) {
        val func_name = FUNC_NAMES(func_index)
        val max = times(func_index).max
        val min = times(func_index).min
        val mean = times(func_index).mean()
        val dev = times(func_index).deviation() / mean * 100
        fo.print(f",$min,$mean,$max")
        println(
          f"$order_str%7s nodes, $size%7d edges, $func_name%8s: $mean%6.3f ± $dev%.0f%%"
        )
      }
      fo.println()
    }
    fo.close()
  }

  implicit class IntContext(val sc: StringContext) {
    def i(args: Any*): Int = {
      val orig = sc.s(args: _*)
      orig.replace("M", "kk").replace("k", "000").toInt
    }
  }

  def main(args: Array[String]): Unit = {
    //noinspection NameBooleanParameters
    assert(false, "Specify -Xdisable-assertions for meaningful measurements")

    val all_func_indices = FUNCS.indices.toArray
    val most_func_indices = FUNCS.indices.slice(1, Int.MaxValue).toArray
    val mt_func_indices = Array(6, 8)
    val sizes_100 = Array(i"2k" to i"3k" by 50: _*)
    val sizes_10k = Array(
      (i"10k" until i"100k" by i"10k")
        ++ (i"100k" to i"200k" by i"25k"): _*
    )
    val sizes_1M = Array(
      (i"50k" until i"250k" by i"50k")
        ++ (i"250k" until i"1M" by i"250k")
        ++ (i"1M" to i"5M" by i"1M"): _*
    )
    bk("100", 100, Array(2000), 3, FUNCS.indices.toArray) // warm up
    Thread.sleep(3210) // give IntelliJ launcher some time to cool down
    bk("100", i"100", sizes_100, 5, all_func_indices)
    bk("10k", i"10k", sizes_10k, 3, most_func_indices)
    bk("1M", i"1M", sizes_1M, 3, mt_func_indices)
  }
}
