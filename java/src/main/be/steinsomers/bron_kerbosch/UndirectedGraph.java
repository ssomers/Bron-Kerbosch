package be.steinsomers.bron_kerbosch;

import java.util.Collections;
import java.util.Comparator;
import java.util.List;
import java.util.Set;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public record UndirectedGraph(List<Set<Integer>> adjacencies) {
    public UndirectedGraph {
        assert IntStream.range(0, adjacencies.size()).noneMatch(v -> adjacencies.get(v).contains(v));
        assert IntStream.range(0, adjacencies.size()).allMatch(v -> adjacencies.get(v).stream().allMatch (
                                                            w -> adjacencies.get(w).contains(v)));
    }

    public int order() {
        return adjacencies.size();
    }

    public int size() {
        final int total = adjacencies.stream().mapToInt(Set::size).sum();
        assert total % 2 == 0;
        return total / 2;
    }

    public int degree(final int node) {
        return adjacencies.get(node).size();
    }

    public boolean hasDegree(final int node) { return !adjacencies.get(node).isEmpty(); }

    public Set<Integer> neighbours(final int node) {
        return Collections.unmodifiableSet(adjacencies.get(node));
    }

    public Stream<Integer> connectedVertices() {
        return IntStream.range(0, order()).filter(this::hasDegree).boxed();
    }

    public int maxDegreeVertex() {
        return IntStream.range(0, order()).boxed()
                .max(Comparator.comparingInt(this::degree))
                .orElseThrow();
    }
}
