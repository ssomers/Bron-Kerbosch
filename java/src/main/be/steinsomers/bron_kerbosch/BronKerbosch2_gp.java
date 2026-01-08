package be.steinsomers.bron_kerbosch;

import java.util.function.Consumer;

public final class BronKerbosch2_gp implements BronKerboschAlgorithm {
    @Override
    public void explore(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer) {
        BronKerboschPivot.explore(graph, cliqueConsumer, PivotChoice.MaxDegreeLocal);
    }
}
