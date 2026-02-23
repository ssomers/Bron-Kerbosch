package be.steinsomers.bron_kerbosch;

import java.util.Collections;
import java.util.List;
import java.util.Set;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class UndirectedGraph {
    private final List<Set<Integer>> my_adjacencies;
    private final int my_size;
    private final int my_max_degree;

    public UndirectedGraph(List<Set<Integer>> adjacencies) {
        assert IntStream.range(0, adjacencies.size()).noneMatch(v -> adjacencies.get(v).contains(v));
        assert IntStream.range(0, adjacencies.size()).allMatch(v -> adjacencies.get(v).stream().allMatch(
                w -> adjacencies.get(w).contains(v)));
        final int total_degree = adjacencies.stream().mapToInt(Set::size).sum();
        assert total_degree % 2 == 0;
        my_adjacencies = adjacencies;
        my_size = total_degree / 2;
        my_max_degree = adjacencies.stream().mapToInt(Set::size).max().orElse(0);
    }

    public int order() {
        return my_adjacencies.size();
    }

    public int size() {
        return my_size;
    }

    public int max_degree() {
        return my_max_degree;
    }

    public int degree(final int vertex) {
        return my_adjacencies.get(vertex).size();
    }

    public boolean hasDegree(final int vertex) {
        return !my_adjacencies.get(vertex).isEmpty();
    }

    public Set<Integer> neighbours(final int vertex) {
        return Collections.unmodifiableSet(my_adjacencies.get(vertex));
    }

    public Stream<Integer> connectedVertices() {
        return IntStream.range(0, order()).filter(this::hasDegree).boxed();
    }

    public Stream<Integer> maxDegreeVertices() {
        return IntStream.range(0, order()).boxed()
                .filter(v -> degree(v) == my_max_degree);
    }
}
