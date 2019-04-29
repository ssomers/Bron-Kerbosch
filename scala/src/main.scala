import scala.util.Random

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
      "Ver3+MT",  
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
    bron_kerbosch3_mt,
  )

  type Clique = bron_kerbosch_algorithm#Clique
  type Cliques = bron_kerbosch_algorithm#Cliques
  def order_cliques(cliques: Cliques): Cliques = {
    cliques
      .map(clique => clique.sortWith(_.compareTo(_) < 0))
      .sortWith((a: Clique, b: Clique) => {
        require(a.size > 1)
        require(b.size > 1)
        require(!(a eq b))
        val result =
          a.iterator.zip(b.iterator).map { case (va, vb) => va - vb }.find {
            _ != 0
          }
        if (result.isEmpty) {
          throw new IllegalArgumentException(
            f"got overlapping or equal cliques $a <> $b"
          )
        }
        result.get < 0
      })
  }

  def bron_kerbosch_timed(graph: UndirectedGraph,
                          samples: Int,
                          func_indices: Array[Int]): Array[SampleStatistics] = {
    var first: Option[Cliques] = None
    val times = Array.fill(FUNCS.size) { new SampleStatistics }

    for (sample <- 1 to samples; func_index <- func_indices) {
      val func = FUNCS(func_index)
      val start = System.currentTimeMillis()

      val cliques = func.explore(graph)
      val elapsed = System.currentTimeMillis() - start
      times(func_index).put(elapsed)

      if (samples > 1 && sample <= 2) {
        val ordered = order_cliques(cliques)
        first match {
          case None               => first = Some(ordered)
          case Some(firstCliques) => require(firstCliques == ordered)
        }
      }
    }
    times
  }

  def bk(order_str: String,
         order: Int,
         sizes: Array[Int],
         samples: Int,
         func_indices: Array[Int]): Unit = {
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
      val times = bron_kerbosch_timed(graph, samples, func_indices)

      fo.print(f"$size")
      for (func_index <- func_indices) {
        val func_name = FUNC_NAMES(func_index)
        val max = times(func_index).max / 1e3
        val min = times(func_index).min / 1e3
        val mean = times(func_index).mean / 1e3
        val dev = times(func_index).deviation / 1e3
        fo.print(f",$min,$mean,$max")
        println(
          f"order $order_str%7s size $size%7d $func_name%8s: $mean%5.2f ±$dev%5.2f"
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

  def main(args: Array[String]) {
    //noinspection NameBooleanParameters
    assert(false, "Specify -Xdisable-assertions for meaningful measurements")

    val sizes_100 = Array(i"2k" to i"3k" by 50: _*)
    val sizes_10k = Array(i"100k" to i"800k" by i"100k": _*)
    val sizes_1M = Array(
      (i"200k" until i"1M" by i"200k")
        ++ (i"1M" to i"3M" by i"1M"): _*
    )
    bk("warm-up", 100, Array(2000), 3, FUNCS.indices.toArray)
    Thread.sleep(4321) // give IntelliJ launcher some time to cool down
    //bk("9999", i"9999", Array(i"567k"), 3, Array(8))
    bk("100", i"100", sizes_100, 5, FUNCS.indices.toArray)
    bk("10k", i"10k", sizes_10k, 3, FUNCS.indices.toArray)
    bk("1M", i"1M", sizes_1M, 3, FUNCS.indices.toArray)
  }
}
