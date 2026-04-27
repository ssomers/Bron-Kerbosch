package be.steinsomers.bron_kerbosch;

@FunctionalInterface
public interface BronKerboschAlgorithm {
    int[] EMPTY_CLIQUE = {};

    void explore(UndirectedGraph graph, CliqueConsumer cliqueConsumer) throws InterruptedException;
}
