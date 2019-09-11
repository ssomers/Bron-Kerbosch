package be.steinsomers.bron_kerbosch;

@FunctionalInterface
public interface BronKerboschAlgorithm {
    void explore(UndirectedGraph graph, Reporter reporter) throws InterruptedException;
}
