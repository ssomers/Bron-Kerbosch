package be.steinsomers.bron_kerbosch;

import java.util.Collections;
import java.util.List;
import java.util.Set;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public final class UndirectedGraph {
    private final List<? extends Set<Integer>> itsAdjacencies;

    public UndirectedGraph(List<Set<Integer>> adjacencies) {
        for (int v = 0; v < adjacencies.size(); ++v) {
            for (int w : adjacencies.get(v)) {
                assert v != w;
                assert adjacencies.get(w).contains(v);
            }
        }
        itsAdjacencies = List.copyOf(adjacencies);
    }

    public int order() {
        return itsAdjacencies.size();
    }

    public int size() {
        int total = IntStream.range(0, order()).map(this::degree).sum();
        assert total % 2 == 0;
        return total / 2;
    }

    public int degree(int node) {
        return itsAdjacencies.get(node).size();
    }

    public Set<Integer> neighbours(int node) {
        return Collections.unmodifiableSet(itsAdjacencies.get(node));
    }

    public Stream<Integer> connectedVertices() {
        return IntStream.range(0, order()).filter(v -> degree(v) > 0).boxed();
    }
}