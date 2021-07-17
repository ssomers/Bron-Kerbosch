import base.Vertex
import main.{Clique, Cliques, FUNCS, FUNC_NAMES, order_cliques}

class BronKerboschTest extends org.scalatest.funsuite.AnyFunSuite {
  def bk(
      adjacency_list: Seq[Seq[Vertex]],
      expected_cliques: Seq[Seq[Vertex]]
  ): Unit = {
    val adjacencies = adjacency_list.map { neighbours =>
      neighbours.toSet
    }.toIndexedSeq
    val graph = new SlimUndirectedGraph(adjacencies)
    for ((func, func_index) <- FUNCS.zipWithIndex) {
      val func_name = FUNC_NAMES(func_index)
      val cliques = new Cliques()
      val reporter = (clique: Clique) => { cliques += clique; () }
      func.explore(graph, reporter)
      val ordered = order_cliques(cliques)
      assert(
        ordered == expected_cliques,
        f"Unexpected result for $func_name: $ordered"
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
    bk(Seq(Seq(1), Seq(0)), Seq(Seq(0, 1)))
  }

  test("order_3_size_1_left") {
    bk(Seq(Seq(1), Seq(0), Seq()), Seq(Seq(0, 1)))
  }

  test("order_3_size_1_long") {
    bk(Seq(Seq(2), Seq(), Seq(0)), Seq(Seq(0, 2)))
  }

  test("order_3_size_1_right") {
    bk(Seq(Seq(), Seq(2), Seq(1)), Seq(Seq(1, 2)))
  }

  test("order_3_size_2") {
    bk(Seq(Seq(1), Seq(0, 2), Seq(1)), Seq(Seq(0, 1), Seq(1, 2)))
  }

  test("order_3_size_3") {
    bk(Seq(Seq(1, 2), Seq(0, 2), Seq(0, 1)), Seq(Seq(0, 1, 2)))
  }

  test("order_4_size_2") {
    bk(Seq(Seq(1), Seq(0), Seq(3), Seq(2)), Seq(Seq(0, 1), Seq(2, 3)))
  }

  test("order_4_size_3_bus") {
    bk(
      Seq(Seq(1), Seq(0, 2), Seq(1, 3), Seq(2)),
      Seq(Seq(0, 1), Seq(1, 2), Seq(2, 3))
    )
  }

  test("order_4_size_3_star") {
    bk(
      Seq(Seq(1, 2, 3), Seq(0), Seq(0), Seq(0)),
      Seq(Seq(0, 1), Seq(0, 2), Seq(0, 3))
    )
  }

  test("order_4_size_4_p") {
    bk(
      Seq(Seq(1), Seq(0, 2, 3), Seq(1, 3), Seq(1, 2)),
      Seq(Seq(0, 1), Seq(1, 2, 3))
    )
  }

  test("order_4_size_4_square") {
    bk(
      Seq(Seq(1, 3), Seq(0, 2), Seq(1, 3), Seq(0, 2)),
      Seq(Seq(0, 1), Seq(0, 3), Seq(1, 2), Seq(2, 3))
    )
  }

  test("order_4_size_5") {
    bk(
      Seq(Seq(1, 2, 3), Seq(0, 2), Seq(0, 1, 3), Seq(0, 2)),
      Seq(Seq(0, 1, 2), Seq(0, 2, 3))
    )
  }

  test("order_4_size_6") {
    bk(
      Seq(Seq(1, 2, 3), Seq(0, 2, 3), Seq(0, 1, 3), Seq(0, 1, 2)),
      Seq(Seq(0, 1, 2, 3))
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
      Seq(Seq(0, 1, 2, 3), Seq(0, 1, 2, 4))
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
      Seq(Seq(1, 2, 3, 4), Seq(2, 3, 5), Seq(5, 6, 7))
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
        Seq(0, 1, 3),
        Seq(0, 1, 6),
        Seq(0, 1, 7),
        Seq(0, 2, 3),
        Seq(0, 2, 7),
        Seq(0, 3, 4),
        Seq(0, 4, 6),
        Seq(0, 4, 7),
        Seq(1, 3, 9),
        Seq(1, 6, 9),
        Seq(1, 7, 9),
        Seq(1, 8),
        Seq(2, 3, 9),
        Seq(2, 5),
        Seq(2, 7, 9),
        Seq(2, 8),
        Seq(3, 4, 9),
        Seq(4, 6, 9),
        Seq(4, 7, 9),
        Seq(5, 6)
      )
    )
  }
}
