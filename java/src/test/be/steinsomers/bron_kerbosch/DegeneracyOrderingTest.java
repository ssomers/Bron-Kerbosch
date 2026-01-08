package be.steinsomers.bron_kerbosch;

import net.jqwik.api.Arbitraries;
import net.jqwik.api.Arbitrary;
import net.jqwik.api.ForAll;
import net.jqwik.api.Property;
import net.jqwik.api.Provide;
import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.NoSuchElementException;
import java.util.Set;
import java.util.SortedSet;
import java.util.TreeSet;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

final class DegeneracyOrderingTest {
    private static SortedSet<Integer> sortedDegeneracyOrdering(final UndirectedGraph g, final int drop) {
        final SortedSet<Integer> vertices = new TreeSet<>();
        new DegeneracyOrdering(g, drop).forEachRemaining((final int v) -> {
            final boolean added = vertices.add(v);
            assertTrue(added);
        });
        return vertices;
    }

    @Test
    void empty() {
        final var g = new UndirectedGraph(List.of());
        assertTrue(sortedDegeneracyOrdering(g, 0).isEmpty());
        assertTrue(sortedDegeneracyOrdering(g, -1).isEmpty());
    }

    @Test
    void pair() {
        final var g = new UndirectedGraph(List.of(Set.of(1), Set.of(0)));
        assertEquals(Set.of(0, 1), sortedDegeneracyOrdering(g, 0));
        assertEquals(1, sortedDegeneracyOrdering(g, -1).size());
        assertEquals(0, sortedDegeneracyOrdering(g, -2).size());
    }

    @Test
    void split() {
        final var g = new UndirectedGraph(List.of(Set.of(1), Set.of(0, 2), Set.of(1)));
        assertNotEquals(1, new DegeneracyOrdering(g, 0).next());
        assertEquals(Set.of(0, 1, 2), sortedDegeneracyOrdering(g, 0));
    }

    /**
     * @param adjacencyLikes List of suggested neighbours, indexed by vertex. The list is oblivious
     *                       to symmetry. If a vertex appears as its own neighbour, that entry will
     *                       be ignored. The list may be empty to begin with. The latter two
     *                       properties make it likely a vertex is unconnected, but the need for
     *                       symmetry makes it likely that another vertex connects to it anyway.
     */
    private static List<Set<Integer>> makeSymmetricAdjacencies(final List<Set<Integer>> adjacencyLikes) {
        final var order = adjacencyLikes.size();
        final List<Set<Integer>> adjacencies = Stream
                .generate(() -> (Set<Integer>) new HashSet<Integer>(16))
                .limit(order)
                .toList();
        for (int v = 0; v < order; ++v) {
            final var neighbours = adjacencyLikes.get(v);
            for (final int w : neighbours) {
                if (v < w) {
                    adjacencies.get(v).add(w);
                    adjacencies.get(w).add(v);
                }
            }
        }
        return adjacencies;
    }

    @Property
    boolean degeneracyOrderingCoversConnectedVertices(
            @ForAll("arbitraryAdjacencyLikes") final List<Set<Integer>> adjacencyLikes) {
        final var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        final var g = new UndirectedGraph(adjacencies);
        final SortedSet<Integer> connectedVertices =
                g.connectedVertices().collect(Collectors.toCollection(TreeSet::new));
        return sortedDegeneracyOrdering(g, 0).equals(connectedVertices);
    }

    @Property
    boolean degeneracyOrderingDrops1(
            @ForAll("arbitraryAdjacencyLikes") final List<Set<Integer>> adjacencyLikes) {
        final var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        final var g = new UndirectedGraph(adjacencies);
        final var ordering = new DegeneracyOrdering(g, 0).stream().toArray();
        final var ordering1 = new DegeneracyOrdering(g, -1).stream().toArray();
        return ordering1.length == Math.max(0, ordering.length - 1)
                && Arrays.equals(ordering, 0, ordering1.length, ordering1, 0, ordering1.length);
    }

    @Property
    boolean degeneracyOrderingStartsWithLowestDegree(
            @ForAll("arbitraryAdjacencyLikes") final List<Set<Integer>> adjacencyLikes) {
        final var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        final var g = new UndirectedGraph(adjacencies);
        final var ordering = new DegeneracyOrdering(g, 0);
        final int first;
        try {
            first = ordering.nextInt();
        } catch (final NoSuchElementException e) {
            return true;
        }
        return ordering.stream().allMatch(v -> g.degree(first) <= g.degree(v));
    }

    private static Arbitrary<Set<Integer>> arbitraryNeighbours(final int order) {
        return Arbitraries.integers().between(0, order - 1).set();
    }

    // Provide arbitrary input for makeSymmetricAdjacencies.
    @Provide
    static Arbitrary<List<Set<Integer>>> arbitraryAdjacencyLikes() {
        final Arbitrary<Integer> order = Arbitraries.integers().between(1, 99);
        return order.flatMap(o -> arbitraryNeighbours(o).list().ofSize(o));
    }
}
