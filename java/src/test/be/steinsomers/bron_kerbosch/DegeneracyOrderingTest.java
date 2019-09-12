package be.steinsomers.bron_kerbosch;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Set;
import java.util.SortedSet;
import java.util.TreeSet;

final class DegeneracyOrderingTest {
    private static SortedSet<Integer> degeneracy_ordering(UndirectedGraph g, int drop) {
        SortedSet<Integer> vertices = new TreeSet<>();
        new DegeneracyOrdering(g, drop).forEachRemaining(v -> {
            boolean added = vertices.add(v);
            Assertions.assertTrue(added);
        });
        return vertices;
    }

    @Test
    void degeneracy_ordering_empty() {
        List<Set<Integer>> adjacencies = List.of();
        var g = new UndirectedGraph(adjacencies);
        Assertions.assertTrue(degeneracy_ordering(g, 0).isEmpty());
        Assertions.assertTrue(degeneracy_ordering(g, -1).isEmpty());
    }

    @Test
    void degeneracy_ordering_pair_unconnected() {
        List<Set<Integer>> adjacencies = List.of(Set.of(), Set.of());
        var g = new UndirectedGraph(adjacencies);
        Assertions.assertTrue(degeneracy_ordering(g, 0).isEmpty());
        Assertions.assertTrue(degeneracy_ordering(g, -1).isEmpty());
    }

    @Test
    void degeneracy_ordering_pair_connected() {
        List<Set<Integer>> adjacencies = List.of(Set.of(1), Set.of(0));
        var g = new UndirectedGraph(adjacencies);
        Assertions.assertEquals(Set.of(0, 1), degeneracy_ordering(g, 0));
        Assertions.assertEquals(1, degeneracy_ordering(g, -1).size());
        Assertions.assertEquals(0, degeneracy_ordering(g, -2).size());
    }
}
