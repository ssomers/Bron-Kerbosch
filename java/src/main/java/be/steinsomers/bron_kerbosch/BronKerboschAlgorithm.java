package be.steinsomers.bron_kerbosch;

import java.util.stream.Stream;

@FunctionalInterface
public interface BronKerboschAlgorithm {
    Stream<int[]> explore(UndirectedGraph graph);
}
