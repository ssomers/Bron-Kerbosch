package be.steinsomers.bron_kerbosch;

import java.util.Collections;
import java.util.Comparator;
import java.util.List;
import java.util.Set;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public record UndirectedGraph(List<Set<Integer>> adjacencies) {
    public UndirectedGraph {
        for (int v = 0; v < adjacencies.size(); ++v) {
            for (int w : adjacencies.get(v)) {
                assert v != w;
                assert adjacencies.get(w).contains(v);
            }
        }
    }

    public int order() {
        return adjacencies.size();
    }

    public int size() {
        int total = adjacencies.stream().mapToInt(Set::size).sum();
        assert total % 2 == 0;
        return total / 2;
    }

    public int degree(int node) {
        return adjacencies.get(node).size();
    }

    public Set<Integer> neighbours(int node) {
        return Collections.unmodifiableSet(adjacencies.get(node));
    }

    public Stream<Integer> connectedVertices() {
        return IntStream.range(0, order()).filter(v -> degree(v) > 0).boxed();
    }

    public int maxDegreeVertex() {
        return IntStream.range(0, order()).boxed()
                .max(Comparator.comparingInt(this::degree))
                .orElseThrow();
    }
}
