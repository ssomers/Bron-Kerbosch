package be.steinsomers.bron_kerbosch;

import java.util.Collection;

@FunctionalInterface
public interface BronKerboschAlgorithm {
    int[] EMPTY_CLIQUE = {};

    Collection<int[]> explore(UndirectedGraph graph) throws InterruptedException;
}
