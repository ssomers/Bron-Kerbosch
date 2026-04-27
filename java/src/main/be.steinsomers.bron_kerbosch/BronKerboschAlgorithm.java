package be.steinsomers.bron_kerbosch;

public interface BronKerboschAlgorithm {
    int[] EMPTY_CLIQUE = {};

    void explore(UndirectedGraph graph, CliqueConsumer cliqueConsumer) throws InterruptedException;
    default boolean hasRaceCondition() {
        return false;
    }
}
