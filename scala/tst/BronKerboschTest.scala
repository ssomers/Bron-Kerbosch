import base.{Clique, Vertex}

import scala.collection.immutable.TreeSet

class BronKerboschTest extends org.scalatest.FunSuite {

    type OrderedClique = TreeSet[Vertex]
    type OrderedCliques = TreeSet[OrderedClique]

    def order_cliques(cliques: List[Clique]): OrderedCliques = {
        val ord = new Ordering[OrderedClique] {
            def compare(a: OrderedClique, b: OrderedClique): Int = {
                a.zip(b).map { case (va, vb) => va - vb }.filter { diff => diff == 0 }.head
            }
        }
        new OrderedCliques()(ord) ++ cliques.map(clique => new OrderedClique ++ clique.toSet).toSet
    }

    def bron_kerbosch(graph: UndirectedGraph): OrderedCliques = {
        /*
        let mut first: Option<OrderedCliques> = None;
        for func in FUNCS {
        */

        val reporter = new SimpleReporter
        bron_kerbosch1.explore(graph, reporter)
        val current = order_cliques(reporter.cliques.toList)
        current /*
            if first.is_none() {
                first = Some(current);
            } else {
                assert_eq!(current, *first.as_ref().unwrap());
            }
        }
        first.unwrap()
            */
    }

    def bk(adjacencylist: List[List[Vertex]], expected_cliques: List[Clique]): Unit = {
        val adjacencies = adjacencylist.map { neighbours => neighbours.toSet }
        val graph = new SlimUndirectedGraph(adjacencies)
        val current = bron_kerbosch(graph)
        assert(current == order_cliques(expected_cliques))
    }

    test("order 0") {
        bk(List(), List())
    }

    /*

    #[test
    ] fn bk_order_1 () {
        bk(vec ![vec ![]
        ], vec !
        [] );
    }

    #[test
    ] fn bk_order_2_isolated () {
        bk(vec ![vec ![]
        , vec !
        []], vec !
        [] );
    }

    #[test
    ] fn bk_order_2_connected () {
        bk(vec ![vec ![1]
        , vec !
        [0
        ]], vec !
        [vec !
        [0
                , 1
        ]] );
    }

    #[test
    ] fn bk_order_3_size_1 () {
        bk(vec ![vec ![1]
        , vec !
        [0
        ], vec !
        []], vec !
        [vec !
        [0
                , 1
        ]] );
        bk(vec ![vec ![]
        , vec !
        [2
        ], vec !
        [1
        ]], vec !
        [vec !
        [1
                , 2
        ]] );
    }

    #[test
    ] fn bk_order_3_size_2 () {
        bk(vec ![vec ![1]
        , vec !
        [0
                , 2
        ], vec !
        [1
        ]], vec !
        [vec !
        [0
                , 1
        ], vec !
        [1
                , 2
        ]], );
    }

    #[test
    ] fn bk_order_3_size_3 () {
        bk(vec ![vec ![1, 2]
        , vec !
        [0
                , 2
        ], vec !
        [0
                , 1
        ]], vec !
        [vec !
        [0
                , 1
                , 2
        ]], );
    }

    #[test
    ] fn bk_order_4_size_2_isolated () {
        bk(vec ![vec ![1, 2]
        , vec !
        [0
        ], vec !
        [0
        ], vec !
        []], vec !
        [vec !
        [0
                , 1
        ], vec !
        [0
                , 2
        ]], );
    }

    #[test
    ] fn bk_order_4_size_2_connected () {
        bk(vec ![vec ![1]
        , vec !
        [0
        ], vec !
        [3
        ], vec !
        [2
        ]], vec !
        [vec !
        [0
                , 1
        ], vec !
        [2
                , 3
        ]], );
    }

    #[test
    ] fn bk_order_4_size_4_p () {
        bk(vec ![vec ![1]
        , vec !
        [0
                , 2
                , 3
        ], vec !
        [1
                , 3
        ], vec !
        [1
                , 2
        ]], vec !
        [vec !
        [0
                , 1
        ], vec !
        [1
                , 2
                , 3
        ]], );
    }

    #[test
    ] fn bk_order_4_size_4_square () {
        bk(vec ![vec ![1, 3]
        , vec !
        [0
                , 2
        ], vec !
        [1
                , 3
        ], vec !
        [0
                , 2
        ]], vec !
        [vec !
        [0
                , 1
        ], vec !
        [0
                , 3
        ], vec !
        [1
                , 2
        ], vec !
        [2
                , 3
        ]], );
    }

    #[test
    ] fn bk_order_4_size_5 () {
        bk(vec ![vec ![1, 2, 3]
        , vec !
        [0
                , 2
        ], vec !
        [0
                , 1
                , 3
        ], vec !
        [0
                , 2
        ]], vec !
        [vec !
        [0
                , 1
                , 2
        ], vec !
        [0
                , 2
                , 3
        ]], );
    }

    #[test
    ] fn bk_order_4_size_6 () {
        bk(vec ![vec ![1, 2, 3]
        , vec !
        [0
                , 2
                , 3
        ], vec !
        [0
                , 1
                , 3
        ], vec !
        [0
                , 1
                , 2
        ]], vec !
        [vec !
        [0
                , 1
                , 2
                , 3
        ]], );
    }

    #[test
    ] fn bk_order_5_penultimate () {
        bk(vec ![vec ![1, 2, 3, 4]
        , vec !
        [0
                , 2
                , 3
                , 4
        ], vec !
        [0
                , 1
                , 3
                , 4
        ], vec !
        [0
                , 1
                , 2
        ], vec !
        [0
                , 1
                , 2
        ], ], vec !
        [vec !
        [0
                , 1
                , 2
                , 3
        ], vec !
        [0
                , 1
                , 2
                , 4
        ]], );
    }

    #[test
    ] fn bk_sample () {
        bk(vec ![vec ![]
        , vec !
        [2
                , 3
                , 4
        ], vec !
        [1
                , 3
                , 4
                , 5
        ], vec !
        [1
                , 2
                , 4
                , 5
        ], vec !
        [1
                , 2
                , 3
        ], vec !
        [2
                , 3
                , 6
                , 7
        ], vec !
        [5
                , 7
        ], vec !
        [5
                , 6
        ], ], vec !
        [vec !
        [1
                , 2
                , 3
                , 4
        ], vec !
        [2
                , 3
                , 5
        ], vec !
        [5
                , 6
                , 7
        ]], );
    }
    */
};
