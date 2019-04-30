import base.Vertex
import main.{Cliques, FUNCS, FUNC_NAMES, order_cliques}

class BronKerboschTest extends org.scalatest.FunSuite {
  def bk(adjacency_list: Seq[Seq[Vertex]], expected_cliques: Cliques): Unit = {
    val adjacencies = adjacency_list.map { neighbours =>
      neighbours.toSet
    }.toIndexedSeq
    val graph = new SlimUndirectedGraph(adjacencies)
    for ((func, func_index) <- FUNCS.zipWithIndex) {
      val func_name = FUNC_NAMES(func_index)
      val cliques = order_cliques(func.explore(graph))
      assert(
        cliques == expected_cliques,
        f"Unexpected result for $func_name: $cliques"
      )
    }
  }

  test("order_0") {
    bk(Seq(), Seq())
  }

  test("order_1") {
    bk(Seq(Seq()), Seq())
  }

  test("order_2_isolated") {
    bk(Seq(Seq(), Seq()), Seq())
  }

  test("order_2_connected") {
    bk(Seq(Seq(1), Seq(0)), Seq(IndexedSeq(0, 1)))
  }

  test("order_3_size_1") {
    bk(Seq(Seq(1), Seq(0), Seq()), Seq(IndexedSeq(0, 1)))
    bk(Seq(Seq(), Seq(2), Seq(1)), Seq(IndexedSeq(1, 2)))
  }

  test("order_3_size_2") {
    bk(Seq(Seq(1), Seq(0, 2), Seq(1)), Seq(IndexedSeq(0, 1), IndexedSeq(1, 2)))
  }

  test("order_3_size_3") {
    bk(Seq(Seq(1, 2), Seq(0, 2), Seq(0, 1)), Seq(IndexedSeq(0, 1, 2)))
  }

  test("order_4_size_2") {
    bk(
      Seq(Seq(1), Seq(0), Seq(3), Seq(2)),
      Seq(IndexedSeq(0, 1), IndexedSeq(2, 3))
    )
  }

  test("order_4_size_3_bus") {
    bk(
      Seq(Seq(1), Seq(0, 2), Seq(1, 3), Seq(2)),
      Seq(IndexedSeq(0, 1), IndexedSeq(1, 2), IndexedSeq(2, 3))
    )
  }

  test("order_4_size_3_star") {
    bk(
      Seq(Seq(1, 2, 3), Seq(0), Seq(0), Seq(0)),
      Seq(IndexedSeq(0, 1), IndexedSeq(0, 2), IndexedSeq(0, 3))
    )
  }

  test("order_4_size_4_p") {
    bk(
      Seq(Seq(1), Seq(0, 2, 3), Seq(1, 3), Seq(1, 2)),
      Seq(IndexedSeq(0, 1), IndexedSeq(1, 2, 3))
    )
  }

  test("order_4_size_4_square") {
    bk(
      Seq(Seq(1, 3), Seq(0, 2), Seq(1, 3), Seq(0, 2)),
      Seq(
        IndexedSeq(0, 1),
        IndexedSeq(0, 3),
        IndexedSeq(1, 2),
        IndexedSeq(2, 3)
      )
    )
  }

  test("order_4_size_5") {
    bk(
      Seq(Seq(1, 2, 3), Seq(0, 2), Seq(0, 1, 3), Seq(0, 2)),
      Seq(IndexedSeq(0, 1, 2), IndexedSeq(0, 2, 3))
    )
  }

  test("order_4_size_6") {
    bk(
      Seq(Seq(1, 2, 3), Seq(0, 2, 3), Seq(0, 1, 3), Seq(0, 1, 2)),
      Seq(IndexedSeq(0, 1, 2, 3))
    )
  }

  test("order_5_penultimate") {
    bk(
      Seq(
        Seq(1, 2, 3, 4),
        Seq(0, 2, 3, 4),
        Seq(0, 1, 3, 4),
        Seq(0, 1, 2),
        Seq(0, 1, 2)
      ),
      Seq(IndexedSeq(0, 1, 2, 3), IndexedSeq(0, 1, 2, 4))
    )
  }

  test("sample") {
    bk(
      Seq(
        Seq(),
        Seq(2, 3, 4),
        Seq(1, 3, 4, 5),
        Seq(1, 2, 4, 5),
        Seq(1, 2, 3),
        Seq(2, 3, 6, 7),
        Seq(5, 7),
        Seq(5, 6)
      ),
      Seq(IndexedSeq(1, 2, 3, 4), IndexedSeq(2, 3, 5), IndexedSeq(5, 6, 7))
    )
  }

  test("bigger") {
    bk(
      Seq(
        Seq(1, 2, 3, 4, 6, 7),
        Seq(0, 3, 6, 7, 8, 9),
        Seq(0, 3, 5, 7, 8, 9),
        Seq(0, 1, 2, 4, 9),
        Seq(0, 3, 6, 7, 9),
        Seq(2, 6),
        Seq(0, 1, 4, 5, 9),
        Seq(0, 1, 2, 4, 9),
        Seq(1, 2),
        Seq(1, 2, 3, 4, 6, 7)
      ),
      Seq(
        IndexedSeq(0, 1, 3),
        IndexedSeq(0, 1, 6),
        IndexedSeq(0, 1, 7),
        IndexedSeq(0, 2, 3),
        IndexedSeq(0, 2, 7),
        IndexedSeq(0, 3, 4),
        IndexedSeq(0, 4, 6),
        IndexedSeq(0, 4, 7),
        IndexedSeq(1, 3, 9),
        IndexedSeq(1, 6, 9),
        IndexedSeq(1, 7, 9),
        IndexedSeq(1, 8),
        IndexedSeq(2, 3, 9),
        IndexedSeq(2, 5),
        IndexedSeq(2, 7, 9),
        IndexedSeq(2, 8),
        IndexedSeq(3, 4, 9),
        IndexedSeq(4, 6, 9),
        IndexedSeq(4, 7, 9),
        IndexedSeq(5, 6)
      )
    )
  }
}
