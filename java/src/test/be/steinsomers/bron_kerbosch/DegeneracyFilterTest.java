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

final class DegeneracyFilterTest {
    private static SortedSet<Integer> sortedDegeneracyFilterIncludingNeighbours(final UndirectedGraph g) {
        final SortedSet<Integer> vertices = new TreeSet<>();
        new DegeneracyFilter(g).forEachRemaining((final int v) -> {
            vertices.add(v);
            vertices.addAll(g.neighbours(v));
        });
        return vertices;
    }

    @Test
    void empty() {
        final var g = new UndirectedGraph(List.of());
        var f = new DegeneracyFilter(g);
        assertFalse(f.hasNext());
    }

    @Test
    void pair() {
        final var g = new UndirectedGraph(List.of(Set.of(1), Set.of(0)));
        var f = new DegeneracyFilter(g);
        assertTrue(f.hasNext());
        f.nextInt();
        assertFalse(f.hasNext());
    }

    @Test
    void split() {
        final var g = new UndirectedGraph(List.of(Set.of(1), Set.of(0, 2), Set.of(1)));
        var f = new DegeneracyFilter(g);
        assertTrue(f.hasNext());
        var first = f.nextInt();
        assertNotEquals(1, first);
        assertTrue(f.hasNext());
        var second = f.nextInt();
        assertNotEquals(first, second);
        assertFalse(f.hasNext());
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
    boolean degeneracyFilterCoversConnectedVertices(
            @ForAll("arbitraryAdjacencyLikes") final List<Set<Integer>> adjacencyLikes) {
        final var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        final var g = new UndirectedGraph(adjacencies);
        final SortedSet<Integer> connectedVertices =
                g.connectedVertices().collect(Collectors.toCollection(TreeSet::new));
        return sortedDegeneracyFilterIncludingNeighbours(g).equals(connectedVertices);
    }

    @Property
    boolean degeneracyFilterDropsSome(
            @ForAll("arbitraryAdjacencyLikes") final List<Set<Integer>> adjacencyLikes) {
        final var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        final var g = new UndirectedGraph(adjacencies);
        final var connected = g.connectedVertices().count();
        final var filtered = new DegeneracyFilter(g).stream().count();
        return connected == 0 ? filtered == connected : filtered < connected;
    }

    @Property
    boolean degeneracyFilterStartsWithLowestDegree(
            @ForAll("arbitraryAdjacencyLikes") final List<Set<Integer>> adjacencyLikes) {
        final var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        final var g = new UndirectedGraph(adjacencies);
        final var f = new DegeneracyFilter(g);
        if (f.hasNext()) {
            final int first = f.nextInt();
            return f.stream().allMatch(v -> g.degree(first) <= g.degree(v));
        } else {
            return true;
        }
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
