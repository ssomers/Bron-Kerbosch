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

final class DegeneracyIteratorTest {
    private static SortedSet<Integer> sortedDegeneracyIncludingNeighbours(final UndirectedGraph g) {
        final SortedSet<Integer> vertices = new TreeSet<>();
        new DegeneracyIterator(g).forEachRemaining((final int v) -> {
            vertices.add(v);
            vertices.addAll(g.neighbours(v));
        });
        return vertices;
    }

    @Test
    void empty() {
        final var g = new UndirectedGraph(List.of());
        var f = new DegeneracyIterator(g);
        assertFalse(f.hasNext());
    }

    @Test
    void pair() {
        final var g = new UndirectedGraph(List.of(Set.of(1), Set.of(0)));
        var f = new DegeneracyIterator(g);
        assertTrue(f.hasNext());
        f.nextInt();
        assertFalse(f.hasNext());
    }

    @Test
    void split() {
        final var g = new UndirectedGraph(List.of(Set.of(1), Set.of(0, 2), Set.of(1)));
        var f = new DegeneracyIterator(g);
        assertTrue(f.hasNext());
        var first = f.nextInt();
        assertNotEquals(1, first);
        assertTrue(f.hasNext());
        var second = f.nextInt();
        assertNotEquals(first, second);
        assertFalse(f.hasNext());
    }

    /**
     * @param adjacencyLikes List of suggested neighbours, where numbers ≥ index are offset by 1.
     *                       Omits the last vertex because we make the actual adjacencies symmetric.
     */
    private static List<Set<Integer>> makeSymmetricAdjacencies(final List<Set<Integer>> adjacencyLikes) {
        final var order = adjacencyLikes.size() + 1;
        final List<Set<Integer>> adjacencies = Stream
                .generate(() -> (Set<Integer>) new HashSet<Integer>(16))
                .limit(order)
                .toList();
        for (int v = 0; v < order - 1; ++v) {
            final var neighbours = adjacencyLikes.get(v);
            for (final int w1 : neighbours) {
                assert w1 < order - 1;
                var w = w1 >= v ? w1 + 1 : w1;
                adjacencies.get(v).add(w);
                adjacencies.get(w).add(v);
            }
        }
        return adjacencies;
    }

    @Property
    boolean degeneracyIteratorCoversConnectedVertices(
            @ForAll("arbitraryAdjacencyLikes") final List<Set<Integer>> adjacencyLikes) {
        final var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        final var g = new UndirectedGraph(adjacencies);
        final SortedSet<Integer> connectedVertices =
                g.connectedVertices().collect(Collectors.toCollection(TreeSet::new));
        return sortedDegeneracyIncludingNeighbours(g).equals(connectedVertices);
    }

    @Property
    boolean degeneracyIteratorDropsSome(
            @ForAll("arbitraryAdjacencyLikes") final List<Set<Integer>> adjacencyLikes) {
        final var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        final var g = new UndirectedGraph(adjacencies);
        final var connected = g.connectedVertices().count();
        final var filtered = new DegeneracyIterator(g).stream().count();
        return connected == 0 ? filtered == connected : filtered < connected;
    }

    @Property
    boolean degeneracyIteratorStartsWithLowestDegree(
            @ForAll("arbitraryAdjacencyLikes") final List<Set<Integer>> adjacencyLikes) {
        final var adjacencies = makeSymmetricAdjacencies(adjacencyLikes);
        final var g = new UndirectedGraph(adjacencies);
        final var f = new DegeneracyIterator(g);
        if (f.hasNext()) {
            final int first = f.nextInt();
            return f.stream().allMatch(v -> g.degree(first) <= g.degree(v));
        } else {
            return true;
        }
    }

    // Provide arbitrary input for makeSymmetricAdjacencies.
    @Provide
    static Arbitrary<List<Set<Integer>>> arbitraryAdjacencyLikes() {
        final Arbitrary<Integer> order = Arbitraries.integers().between(2, 12);
        return order.flatMap(o -> arbitraryNeighbourLikes(o).list().ofSize(o - 1));
    }

    private static Arbitrary<Set<Integer>> arbitraryNeighbourLikes(final int order) {
        return Arbitraries.integers().between(0, order - 2).set();
    }
}
