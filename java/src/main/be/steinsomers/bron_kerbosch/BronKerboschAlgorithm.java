package be.steinsomers.bron_kerbosch;

import java.util.function.Consumer;

@FunctionalInterface
public interface BronKerboschAlgorithm {
    int[] EMPTY_CLIQUE = {};

    void explore(UndirectedGraph graph, Consumer<int[]> cliqueConsumer) throws InterruptedException;
}
