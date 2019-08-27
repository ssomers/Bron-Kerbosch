package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.UndirectedGraph;
import be.steinsomers.bron_kerbosch.util;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.Random;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

class RandomGraphGenerator {
    private static ArrayList<HashSet<Integer>> new_adjacencies(int n) {
        ArrayList<HashSet<Integer>> adjacencies = new ArrayList<>(n);
        IntStream.range(0, n).forEach(i -> adjacencies.add(new HashSet<>()));
        return adjacencies;
    }

    private void add_edge(int v, int w) {
        final var order = adjacency_sets.size();
        var neighbours = adjacency_sets.get(v);
        var ac = adjacency_complements.get(v);
        neighbours.add(w);
        if (neighbours.size() == order - 1) {
            util.remove_from(unsaturated_vertices, v);
            ac.clear();
        } else if (neighbours.size() == order / 2) {
            // start using adjacency complement
            assert ac.isEmpty();
            ac.addAll(unsaturated_vertices);
            ac.remove(v);
            ac.removeAll(neighbours);
            assert !adjacency_complements.get(v).isEmpty();
        } else if (neighbours.size() > order / 2) {
            var ok = ac.remove(w);
            assert ok;
            assert !adjacency_complements.get(v).isEmpty();
        }
    }

    private ArrayList<Integer> unsaturated_vertices;
    private ArrayList<HashSet<Integer>> adjacency_sets;
    private ArrayList<HashSet<Integer>> adjacency_complements;

    public UndirectedGraph new_undirected(Random rng, int order, int size) {
        var fully_meshed_size = ((long) order) * ((long) order - 1) / 2;
        if (size > fully_meshed_size) {
            throw new IllegalArgumentException(String.format("%d nodes accommodate at most %d edges",
                    order, fully_meshed_size));
        }
        unsaturated_vertices = new ArrayList<>(order);
        adjacency_sets = new_adjacencies(order);
        adjacency_complements = new_adjacencies(order);
        IntStream.range(0, order).forEach(v -> unsaturated_vertices.add(v));
        for (int i = 0; i < size; ++i) {
            assert unsaturated_vertices.stream()
                    .allMatch(v -> adjacency_sets.get(v).size() < order - 1);
            int v = util.random_choice(rng, unsaturated_vertices);
            var ac = adjacency_complements.get(v);
            int w;
            if (ac.isEmpty()) {
                var neighbours = adjacency_sets.get(v);
                w = v;
                while (w == v || neighbours.contains(w)) {
                    w = util.random_choice(rng, unsaturated_vertices);
                }
            } else {
                w = util.random_sample(rng, ac);
            }
            assert v != w;
            assert !adjacency_sets.get(v).contains(w);
            assert !adjacency_sets.get(w).contains(v);
            add_edge(v, w);
            add_edge(w, v);
        }
        var adjacencies = adjacency_sets.stream().map(Set::copyOf).collect(Collectors.toList());
        var g = new UndirectedGraph(adjacencies);
        if (g.order() != order) throw new RuntimeException("order mishap");
        if (g.size() != size) throw new RuntimeException("size mishap");
        return g;
    }
}
