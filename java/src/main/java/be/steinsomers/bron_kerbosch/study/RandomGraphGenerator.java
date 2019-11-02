package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.UndirectedGraph;
import be.steinsomers.bron_kerbosch.util;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Random;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

@SuppressWarnings("CollectionWithoutInitialCapacity")
final class RandomGraphGenerator {
    private static List<Set<Integer>> new_sets(int n) {
        return Stream.generate(() -> new HashSet<Integer>()).limit(n).collect(Collectors.toList());
    }

    private final Random rng;

    RandomGraphGenerator(Random rng) {
        this.rng = rng;
    }

    private int pickNewNeighbour(int v) {
        var ac = adjacencyComplements.get(v);
        if (ac.isEmpty()) {
            // not yet using adjacency complement for this vertex
            var neighbours = adjacencySets.get(v);
            int w = v;
            while (w == v || neighbours.contains(w)) {
                w = util.RandomChoice(rng, unsaturatedVertices);
            }
            return w;
        } else {
            return util.RandomSample(rng, ac);
        }
    }

    private void addEdge(int v, int w) {
        final var order = adjacencySets.size();
        assert order / 2 < order - 1; // for readability and otherwise nothing to randomize anyways
        var neighbours = adjacencySets.get(v);
        neighbours.add(w);
        if (neighbours.size() >= order / 2) {
            var ac = adjacencyComplements.get(v);
            if (neighbours.size() == order / 2) {
                // start using adjacency complement
                assert ac.isEmpty();
                unsaturatedVertices.stream()
                        .filter(u -> u != v && !neighbours.contains(u))
                        .forEach(ac::add);
            } else if (neighbours.size() < order - 1) {
                // continue using adjacency complement
                var ok = ac.remove(w);
                assert ok;
            } else {
                // stop using vertex entirely
                util.RemoveFrom(unsaturatedVertices, v);
                ac.clear(); // clean up, may help or harm performance
            }
        }
    }

    private ArrayList<Integer> unsaturatedVertices;
    private List<Set<Integer>> adjacencySets;
    private List<Set<Integer>> adjacencyComplements;

    UndirectedGraph newUndirected(int order, int size) {
        assert order > 2;
        assert size >= 0;
        var fullyMeshedSize = ((long) order) * (order - 1) / 2;
        if (size > fullyMeshedSize) {
            throw new IllegalArgumentException(String.format(
                    "%d nodes accommodate at most %d edges", order, fullyMeshedSize));
        }

        unsaturatedVertices = IntStream.range(0, order).boxed()
                .collect(Collectors.toCollection(ArrayList::new));
        adjacencySets = new_sets(order);
        adjacencyComplements = new_sets(order);
        for (int i = 0; i < size; ++i) {
            assert unsaturatedVertices.stream()
                    .allMatch(v -> adjacencySets.get(v).size() < order - 1);
            int v = util.RandomChoice(rng, unsaturatedVertices);
            int w = pickNewNeighbour(v);
            assert v != w;
            assert !adjacencySets.get(v).contains(w);
            assert !adjacencySets.get(w).contains(v);
            addEdge(v, w);
            addEdge(w, v);
        }
        var adjacencies = adjacencySets.stream().map(Set::copyOf).collect(Collectors.toList());
        var g = new UndirectedGraph(adjacencies);
        if (g.order() != order) throw new AssertionError("order mishap");
        if (g.size() != size) throw new AssertionError("size mishap");
        return g;
    }
}
