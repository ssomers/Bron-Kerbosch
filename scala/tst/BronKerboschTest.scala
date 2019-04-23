import base.Vertex
import main.{Cliques, FUNCS, FUNC_NAMES, order_cliques}

class BronKerboschTest extends org.scalatest.FunSuite {
  def bk(adjacency_list: List[List[Vertex]],
         expected_cliques: Cliques): Unit = {
    val adjacencies = adjacency_list.map { neighbours =>
      neighbours.toSet
    }.toIndexedSeq
    val graph = new SlimUndirectedGraph(adjacencies)
    for ((func, func_index) <- FUNCS.zipWithIndex) {
      val func_name = FUNC_NAMES(func_index)
      val reporter = new SimpleReporter
      func.explore(graph, reporter)
      val cliques = order_cliques(reporter.cliques.toList)
      assert(
        cliques == expected_cliques,
        f"Unexpected result for $func_name: $cliques"
      )
    }
  }

  test("order_0") {
    bk(List(), List())
  }

  test("order_1") {
    bk(List(List()), List())
  }

  test("order_2_isolated") {
    bk(List(List(), List()), List())
  }

  test("order_2_connected") {
    bk(List(List(1), List(0)), List(IndexedSeq(0, 1)))
  }

  test("order_3_size_1") {
    bk(List(List(1), List(0), List()), List(IndexedSeq(0, 1)))
    bk(List(List(), List(2), List(1)), List(IndexedSeq(1, 2)))
  }

  test("order_3_size_2") {
    bk(
      List(List(1), List(0, 2), List(1)),
      List(IndexedSeq(0, 1), IndexedSeq(1, 2))
    )
  }

  test("order_3_size_3") {
    bk(List(List(1, 2), List(0, 2), List(0, 1)), List(IndexedSeq(0, 1, 2)))
  }

  test("order_4_size_2") {
    bk(
      List(List(1), List(0), List(3), List(2)),
      List(IndexedSeq(0, 1), IndexedSeq(2, 3))
    )
  }

  test("order_4_size_3_bus") {
    bk(
      List(List(1), List(0, 2), List(1, 3), List(2)),
      List(IndexedSeq(0, 1), IndexedSeq(1, 2), IndexedSeq(2, 3))
    )
  }

  test("order_4_size_3_star") {
    bk(
      List(List(1, 2, 3), List(0), List(0), List(0)),
      List(IndexedSeq(0, 1), IndexedSeq(0, 2), IndexedSeq(0, 3))
    )
  }

  test("order_4_size_4_p") {
    bk(
      List(List(1), List(0, 2, 3), List(1, 3), List(1, 2)),
      List(IndexedSeq(0, 1), IndexedSeq(1, 2, 3))
    )
  }

  test("order_4_size_4_square") {
    bk(
      List(List(1, 3), List(0, 2), List(1, 3), List(0, 2)),
      List(
        IndexedSeq(0, 1),
        IndexedSeq(0, 3),
        IndexedSeq(1, 2),
        IndexedSeq(2, 3)
      )
    )
  }

  test("order_4_size_5") {
    bk(
      List(List(1, 2, 3), List(0, 2), List(0, 1, 3), List(0, 2)),
      List(IndexedSeq(0, 1, 2), IndexedSeq(0, 2, 3))
    )
  }

  test("order_4_size_6") {
    bk(
      List(List(1, 2, 3), List(0, 2, 3), List(0, 1, 3), List(0, 1, 2)),
      List(IndexedSeq(0, 1, 2, 3))
    )
  }

  test("order_5_penultimate") {
    bk(
      List(
        List(1, 2, 3, 4),
        List(0, 2, 3, 4),
        List(0, 1, 3, 4),
        List(0, 1, 2),
        List(0, 1, 2)
      ),
      List(IndexedSeq(0, 1, 2, 3), IndexedSeq(0, 1, 2, 4))
    )
  }

  test("sample") {
    bk(
      List(
        List(),
        List(2, 3, 4),
        List(1, 3, 4, 5),
        List(1, 2, 4, 5),
        List(1, 2, 3),
        List(2, 3, 6, 7),
        List(5, 7),
        List(5, 6)
      ),
      List(IndexedSeq(1, 2, 3, 4), IndexedSeq(2, 3, 5), IndexedSeq(5, 6, 7))
    )
  }

  test("bigger") {
    bk(
      List(
        List(1, 2, 3, 4, 6, 7),
        List(0, 3, 6, 7, 8, 9),
        List(0, 3, 5, 7, 8, 9),
        List(0, 1, 2, 4, 9),
        List(0, 3, 6, 7, 9),
        List(2, 6),
        List(0, 1, 4, 5, 9),
        List(0, 1, 2, 4, 9),
        List(1, 2),
        List(1, 2, 3, 4, 6, 7)
      ),
      List(
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
