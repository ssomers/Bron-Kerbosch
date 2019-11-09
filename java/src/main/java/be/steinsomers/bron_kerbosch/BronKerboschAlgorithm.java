package be.steinsomers.bron_kerbosch;

import java.util.Collection;

@FunctionalInterface
public interface BronKerboschAlgorithm {
    Collection<Collection<Integer>> explore(UndirectedGraph graph) throws InterruptedException;
}
