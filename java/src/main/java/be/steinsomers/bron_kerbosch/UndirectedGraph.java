package be.steinsomers.bron_kerbosch;

import java.util.Collections;
import java.util.Comparator;
import java.util.List;
import java.util.Set;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class UndirectedGraph {
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

    public final int order() {
        return itsAdjacencies.size();
    }

    public final int size() {
        int total = IntStream.range(0, order()).map(this::degree).sum();
        assert total % 2 == 0;
        return total / 2;
    }

    public final int degree(int node) {
        return itsAdjacencies.get(node).size();
    }

    public final Set<Integer> neighbours(int node) {
        return Collections.unmodifiableSet(itsAdjacencies.get(node));
    }

    public final Stream<Integer> connectedVertices() {
        return IntStream.range(0, order()).filter(v -> degree(v) > 0).boxed();
    }

    public final int maxDegreeVertex() {
        return IntStream.range(0, order()).boxed()
                .max(Comparator.comparingInt(this::degree))
                .orElseThrow();
    }
}
