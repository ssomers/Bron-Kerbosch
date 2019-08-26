package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.UndirectedGraph;

import java.util.*;
import java.util.stream.IntStream;

public class RandomGraphGenerator {
    public static int random_choice(Random rng, List<Integer> list) {
        var i = rng.nextInt(list.size());
        return list.get(i);
    }

    public static int random_sample(Random rng, Set<Integer> set) {
        var i = rng.nextInt(set.size());
        return set.stream().skip(i).findFirst().orElseThrow();
    }

    public static void remove_from(ArrayList<Integer> list, int v) {
        var last = list.size() - 1;
        var i = list.indexOf(v);
        list.set(i, list.get(last));
        list.remove(last);
    }

    public static ArrayList<HashSet<Integer>> new_adjacencies(int n) {
        ArrayList<HashSet<Integer>> adjacencies = new ArrayList<>(n);
        IntStream.range(0, n).forEach((int i) -> adjacencies.add(new HashSet<>()));
        return adjacencies;
    }

    public static UndirectedGraph new_undirected(Random rng, int order, int size) {
        assert order > 0;
        var fully_meshed_size = ((long) order) * ((long) order - 1) / 2;
        if (size > fully_meshed_size) {
            throw new IllegalArgumentException(String.format("%d nodes accommodate at most %d edges",
                    order, fully_meshed_size));
        }
        var unsaturated_vertices = new ArrayList<Integer>(order);
        IntStream.range(0, order).forEach((int v) -> unsaturated_vertices.add(v));
        var adjacency_sets = new_adjacencies(order);
        var adjacency_complements = new_adjacencies(order);
        for (int i = 0; i < size; ++i) {
            assert unsaturated_vertices.stream()
                    .allMatch((Integer v) -> adjacency_sets.get(v).size() < order - 1);
            int v = random_choice(rng, unsaturated_vertices);
            int w;
            if (adjacency_complements.get(v).isEmpty()) {
                w = v;
                while (w == v || adjacency_sets.get(v).contains(w)) {
                    w = random_choice(rng, unsaturated_vertices);
                }
            } else {
                w = random_sample(rng, adjacency_complements.get(v));
            }
            assert v != w;
            assert !adjacency_sets.get(v).contains(w);
            assert !adjacency_sets.get(w).contains(v);
            for (int j = 0; j < 2; ++j) {
                adjacency_sets.get(v).add(w);
                var neighbours = adjacency_sets.get(v).size();
                if (neighbours == order - 1) {
                    remove_from(unsaturated_vertices, v);
                    adjacency_complements.get(v).clear();
                } else if (neighbours == order / 2) {
                    // start using adjacency complement
                    var ac = adjacency_complements.get(v);
                    assert ac.isEmpty();
                    ac.addAll(unsaturated_vertices);
                    ac.remove(v);
                    ac.removeAll(adjacency_sets.get(v));
                } else if (neighbours > order / 2) {
                    var ok = adjacency_complements.get(v).remove(w);
                    assert ok;
                }
                var t = v;
                v = w;
                w = t;
            }
        }
        var adjacencies = adjacency_sets;
        var g = new UndirectedGraph(adjacencies);
        assert g.order() == order;
        assert g.size() == size;
        return g;
    }
}
