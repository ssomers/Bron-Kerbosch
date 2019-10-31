package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.SimpleReporter;
import be.steinsomers.bron_kerbosch.UndirectedGraph;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.Collection;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

final class BronKerboschTest {
    private static void bk(Collection<List<Integer>> adjacenciesList,
                           List<List<Integer>> expectedCliques) {
        var adjacencies = adjacenciesList.stream().map(Set::copyOf).collect(Collectors.toList());
        var graph = new UndirectedGraph(adjacencies);
        for (int funcIndex = 0; funcIndex < Main.FUNCS.length; ++funcIndex) {
            var funcName = Main.FUNC_NAMES[funcIndex];
            var reporter = new SimpleReporter();
            try {
                Main.FUNCS[funcIndex].explore(graph, reporter);
            } catch (InterruptedException ex) {
                throw new AssertionError(ex);
            }
            var cliques = Main.OrderCliques(reporter.cliques);
            Assertions.assertEquals(cliques, expectedCliques,
                    String.format("Unexpected result for %s", funcName));
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
    void order_3_size_1_left() {
        bk(List.of(List.of(1), List.of(0), List.of()), List.of(List.of(0, 1)));
    }

    @Test
    void order_3_size_1_middle() {
        bk(List.of(List.of(2), List.of(), List.of(0)), List.of(List.of(0, 2)));
    }

    @Test
    void order_3_size_1_right() {
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
        bk(
                List.of(List.of(1), List.of(0), List.of(3), List.of(2)),
                List.of(List.of(0, 1), List.of(2, 3))
        );
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
