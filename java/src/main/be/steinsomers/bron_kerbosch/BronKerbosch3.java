package be.steinsomers.bron_kerbosch;

import java.util.function.Consumer;

public final class BronKerbosch3  implements BronKerboschAlgorithm {
    @Override
    public void explore(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer) {
        BronKerboschOrder.explore(graph, cliqueConsumer, PivotChoice.Arbitrary);
    }
}
