import base.{Clique, Vertex}
import main.{OrderedCliques, order_cliques}

class BronKerboschTest extends org.scalatest.FunSuite {
  def bron_kerbosch(graph: UndirectedGraph): OrderedCliques = {
    /*
        let mut first: Option<OrderedCliques> = None;
        for func in FUNCS {
     */
    val reporter = new SimpleReporter
    bron_kerbosch1.explore(graph, reporter)
    val cliques = reporter.cliques.toList
    order_cliques(cliques) /*
            if first.is_none() {
                first = Some(current);
            } else {
                assert_eq!(current, *first.as_ref().unwrap());
            }
        }
        first.unwrap()
   */
  }

  def bk(adjacencylist: List[List[Vertex]],
         expected_cliques: List[Clique]): Unit = {
    val adjacencies = adjacencylist.map { neighbours =>
      neighbours.toSet
    }
    val graph = new SlimUndirectedGraph(adjacencies)
    val current = bron_kerbosch(graph)
    assert(current == order_cliques(expected_cliques))
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
    bk(List(List(1), List(0)), List(List(0, 1)))
  }

  test("order_3_size_1") {
    bk(List(List(1), List(0), List()), List(List(0, 1)))
    bk(List(List(), List(2), List(1)), List(List(1, 2)))
  }

  test("order_3_size_2") {
    bk(List(List(1), List(0, 2), List(1)), List(List(0, 1), List(1, 2)))
  }

  test("order_3_size_3") {
    bk(List(List(1, 2), List(0, 2), List(0, 1)), List(List(0, 1, 2)))
  }

  test("order_4_size_2_isolated") {
    bk(List(List(1, 2), List(0), List(0), List()), List(List(0, 1), List(0, 2)))
  }

  test("order_4_size_2_connected") {
    bk(List(List(1), List(0), List(3), List(2)), List(List(0, 1), List(2, 3)))
  }

  test("order_4_size_4_p") {
    bk(
      List(List(1), List(0, 2, 3), List(1, 3), List(1, 2)),
      List(List(0, 1), List(1, 2, 3))
    )
  }

  test("order_4_size_4_square") {
    bk(
      List(List(1, 3), List(0, 2), List(1, 3), List(0, 2)),
      List(List(0, 1), List(0, 3), List(1, 2), List(2, 3))
    )
  }

  test("order_4_size_5") {
    bk(
      List(List(1, 2, 3), List(0, 2), List(0, 1, 3), List(0, 2)),
      List(List(0, 1, 2), List(0, 2, 3))
    )
  }

  test("order_4_size_6") {
    bk(
      List(List(1, 2, 3), List(0, 2, 3), List(0, 1, 3), List(0, 1, 2)),
      List(List(0, 1, 2, 3))
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
      List(List(0, 1, 2, 3), List(0, 1, 2, 4))
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
      List(List(1, 2, 3, 4), List(2, 3, 5), List(5, 6, 7))
    )
  }
}
