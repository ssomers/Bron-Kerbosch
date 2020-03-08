package be.steinsomers.bron_kerbosch;

import java.util.stream.Stream;

@FunctionalInterface
public interface BronKerboschAlgorithm {
    int[] EMPTY_CLIQUE = {};

    Stream<int[]> explore(UndirectedGraph graph) throws InterruptedException;
}
