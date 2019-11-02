package be.steinsomers.bron_kerbosch;

import net.jqwik.api.Arbitraries;
import net.jqwik.api.Arbitrary;
import net.jqwik.api.ForAll;
import net.jqwik.api.Property;
import net.jqwik.api.Provide;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.SortedSet;
import java.util.TreeSet;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

final class DegeneracyOrderingTest {
    private static SortedSet<Integer> degeneracyOrdering(UndirectedGraph g, int drop) {
        SortedSet<Integer> vertices = new TreeSet<>();
        new DegeneracyOrdering(g, drop).forEachRemaining((int v) -> {
            boolean added = vertices.add(v);
            assertTrue(added);
        });
        return vertices;
    }

    @Test
    void empty() {
        var g = new UndirectedGraph(List.of());
        assertTrue(degeneracyOrdering(g, 0).isEmpty());
        assertTrue(degeneracyOrdering(g, -1).isEmpty());
    }

    @Test
    void pair() {
        var g = new UndirectedGraph(List.of(Set.of(1), Set.of(0)));
        assertEquals(Set.of(0, 1), degeneracyOrdering(g, 0));
        assertEquals(1, degeneracyOrdering(g, -1).size());
        assertEquals(0, degeneracyOrdering(g, -2).size());
    }

    @Test
    void split() {
        var g = new UndirectedGraph(List.of(Set.of(1), Set.of(0, 2), Set.of(1)));
        assertNotEquals(1, new DegeneracyOrdering(g, 0).next());
        assertEquals(Set.of(0, 1, 2), degeneracyOrdering(g, 0));
    }

    private static List<Set<Integer>> makeSymmetricAdjacencies(List<Set<Integer>> adjacencyLikes) {
        final var order = adjacencyLikes.size();
        final List<Set<Integer>> adjacencies = Stream
                .generate(() -> new HashSet<Integer>())
                .limit(order)
                .collect(Collectors.toList());
        for (int v = 0; v < order; ++v) {
            var neighbours = adjacencyLikes.get(v);
            for (int w : neighbours) {
                if (v != w) {
                    adjacencies.get(v).add(w);
                    adjacencies.get(w).add(v);
                }
            }
        }
        return adjacencies;
    }

    @Property
    boolean degeneracyOrderingCoversConnectedVertices(
            @ForAll("arbitraryAdjacencyLikes") List<Set<Integer>> adjacencyLikes) {
        var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        var g = new UndirectedGraph(adjacencies);
        SortedSet<Integer> connectedVertices =
                g.connectedVertices().collect(Collectors.toCollection(TreeSet::new));
        return degeneracyOrdering(g, 0).equals(connectedVertices);
    }

    @Property
    boolean degeneracyOrderingDrops1(
            @ForAll("arbitraryAdjacencyLikes") List<Set<Integer>> adjacencyLikes) {
        var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        var g = new UndirectedGraph(adjacencies);
        return degeneracyOrdering(g, -1).size() == Math.max(0, g.connectedVertices().count() - 1);
    }

    private static Arbitrary<Set<Integer>> arbitraryNeighbours(int order) {
        return Arbitraries.integers().between(0, order - 1).set();
    }

    @SuppressWarnings({"WeakerAccess", "unused"})
    @Provide
    private static Arbitrary<List<Set<Integer>>> arbitraryAdjacencyLikes() {
        Arbitrary<Integer> orders = Arbitraries.integers().between(1, 99);
        return orders.flatMap(order -> arbitraryNeighbours(order).list().ofSize(order));
    }
}
