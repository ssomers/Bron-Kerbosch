package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.SimpleReporter;
import be.steinsomers.bron_kerbosch.UndirectedGraph;
import be.steinsomers.bron_kerbosch.util;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.List;
import java.util.stream.Collectors;

class BronKerboschTest {
    private void bk(List<List<Integer>> adjacency_list,
                    List<List<Integer>> expected_cliques) {
        var adjacencies = adjacency_list.stream().map(HashSet::new).collect(Collectors.toList());
        var graph = new UndirectedGraph(adjacencies);
        for (int func_index = 0; func_index < Main.FUNCS.length; ++func_index) {
            var func_name = Main.FUNC_NAMES[func_index];
            var reporter = new SimpleReporter();
            Main.FUNCS[func_index].explore(graph, reporter);
            var cliques = util.OrderCliques(reporter.cliques);
            Assertions.assertEquals(cliques, expected_cliques,
                    String.format("Unexpected result for %s", func_name));
        }
    }

    @Test
    void order_0() {
        bk(List.of(), List.of());
    }

    @Test
    void order_1() {
        bk(List.of(List.of()), List.of());
    }

    @Test
    void order_2_isolated() {
        bk(List.of(List.of(), List.of()), List.of());
    }

    @Test
    void order_2_connected() {
        bk(List.of(List.of(1), List.of(0)), List.of(List.of(0, 1)));
    }

    @Test
    void order_3_size_1() {
        bk(List.of(List.of(1), List.of(0), List.of()), List.of(List.of(0, 1)));
        bk(List.of(List.of(), List.of(2), List.of(1)), List.of(List.of(1, 2)));
    }

    @Test
    void order_3_size_2() {
        bk(List.of(List.of(1), List.of(0, 2), List.of(1)), List.of(List.of(0, 1), List.of(1, 2)));
    }

    @Test
    void order_3_size_3() {
        bk(List.of(List.of(1, 2), List.of(0, 2), List.of(0, 1)), List.of(List.of(0, 1, 2)));
    }

    @Test
    void order_4_size_2() {
        bk(List.of(List.of(1), List.of(0), List.of(3), List.of(2)), List.of(List.of(0, 1), List.of(2, 3)));
    }

    @Test
    void order_4_size_3_bus() {
        bk(
                List.of(List.of(1), List.of(0, 2), List.of(1, 3), List.of(2)),
                List.of(List.of(0, 1), List.of(1, 2), List.of(2, 3))
        );
    }

    @Test
    void order_4_size_3_star() {
        bk(
                List.of(List.of(1, 2, 3), List.of(0), List.of(0), List.of(0)),
                List.of(List.of(0, 1), List.of(0, 2), List.of(0, 3))
        );
    }

    @Test
    void order_4_size_4_p() {
        bk(
                List.of(List.of(1), List.of(0, 2, 3), List.of(1, 3), List.of(1, 2)),
                List.of(List.of(0, 1), List.of(1, 2, 3))
        );
    }

    @Test
    void order_4_size_4_square() {
        bk(
                List.of(List.of(1, 3), List.of(0, 2), List.of(1, 3), List.of(0, 2)),
                List.of(List.of(0, 1), List.of(0, 3), List.of(1, 2), List.of(2, 3))
        );
    }

    @Test
    void order_4_size_5() {
        bk(
                List.of(List.of(1, 2, 3), List.of(0, 2), List.of(0, 1, 3), List.of(0, 2)),
                List.of(List.of(0, 1, 2), List.of(0, 2, 3))
        );
    }

    @Test
    void order_4_size_6() {
        bk(
                List.of(List.of(1, 2, 3), List.of(0, 2, 3), List.of(0, 1, 3), List.of(0, 1, 2)),
                List.of(List.of(0, 1, 2, 3))
        );
    }

    @Test
    void order_5_penultimate() {
        bk(
                List.of(
                        List.of(1, 2, 3, 4),
                        List.of(0, 2, 3, 4),
                        List.of(0, 1, 3, 4),
                        List.of(0, 1, 2),
                        List.of(0, 1, 2)
                ),
                List.of(List.of(0, 1, 2, 3), List.of(0, 1, 2, 4))
        );
    }

    @Test
    void sample() {
        bk(
                List.of(
                        List.of(),
                        List.of(2, 3, 4),
                        List.of(1, 3, 4, 5),
                        List.of(1, 2, 4, 5),
                        List.of(1, 2, 3),
                        List.of(2, 3, 6, 7),
                        List.of(5, 7),
                        List.of(5, 6)
                ),
                List.of(List.of(1, 2, 3, 4), List.of(2, 3, 5), List.of(5, 6, 7))
        );
    }

    @Test
    void bigger() {
        bk(
                List.of(
                        List.of(1, 2, 3, 4, 6, 7),
                        List.of(0, 3, 6, 7, 8, 9),
                        List.of(0, 3, 5, 7, 8, 9),
                        List.of(0, 1, 2, 4, 9),
                        List.of(0, 3, 6, 7, 9),
                        List.of(2, 6),
                        List.of(0, 1, 4, 5, 9),
                        List.of(0, 1, 2, 4, 9),
                        List.of(1, 2),
                        List.of(1, 2, 3, 4, 6, 7)
                ),
                List.of(
                        List.of(0, 1, 3),
                        List.of(0, 1, 6),
                        List.of(0, 1, 7),
                        List.of(0, 2, 3),
                        List.of(0, 2, 7),
                        List.of(0, 3, 4),
                        List.of(0, 4, 6),
                        List.of(0, 4, 7),
                        List.of(1, 3, 9),
                        List.of(1, 6, 9),
                        List.of(1, 7, 9),
                        List.of(1, 8),
                        List.of(2, 3, 9),
                        List.of(2, 5),
                        List.of(2, 7, 9),
                        List.of(2, 8),
                        List.of(3, 4, 9),
                        List.of(4, 6, 9),
                        List.of(4, 7, 9),
                        List.of(5, 6)
                )
        );
    }
}
