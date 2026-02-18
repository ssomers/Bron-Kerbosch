package be.steinsomers.bron_kerbosch

import net.jqwik.api.*
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test
import java.util.*
import java.util.function.IntConsumer
import java.util.stream.Stream

internal class DegeneracyFilterTest {
    private fun sortedDegeneracyOrderingIncludingNeighbours(g: UndirectedGraph): SortedSet<Int> {
        val vertices: SortedSet<Int> = TreeSet()
        DegeneracyFilter(g).forEachRemaining(IntConsumer { v: Int ->
            vertices.add(v)
            vertices.addAll(g.neighbours(v))
        })
        return vertices
    }

    /**
     * @param adjacencyLikes List of suggested neighbours, indexed by vertex. The list is oblivious
     * to symmetry. If a vertex appears as its own neighbour, that entry will
     * be ignored. The list may be empty to begin with. The latter two
     * properties make it likely a vertex is unconnected, but the need for
     * symmetry makes it likely that another vertex connects to it anyway.
     */
    private fun makeSymmetricAdjacencies(adjacencyLikes: List<Set<Int>>): List<Set<Int>> {
        val order = adjacencyLikes.size
        val adjacencies = Stream
            .generate { HashSet<Int>(16) as MutableSet<Int> }
            .limit(order.toLong())
            .toList()
        for (v in 0..<order) {
            val neighbours = adjacencyLikes[v]
            for (w in neighbours) {
                if (v < w) {
                    adjacencies[v].add(w)
                    adjacencies[w].add(v)
                }
            }
        }
        return adjacencies
    }

    @Test
    fun empty() {
        val g = UndirectedGraph(listOf())
        val f = DegeneracyFilter(g)
        Assertions.assertFalse(f.hasNext())
    }

    @Test
    fun pair() {
        val g = UndirectedGraph(listOf(setOf(1), setOf(0)))
        val f = DegeneracyFilter(g)
        Assertions.assertTrue(f.hasNext())
        f.nextInt()
        Assertions.assertFalse(f.hasNext())
    }

    @Test
    fun split() {
        val g = UndirectedGraph(listOf(setOf(1), setOf(0, 2), setOf(1)))
        val f = DegeneracyFilter(g)
        Assertions.assertTrue(f.hasNext())
        val first = f.nextInt()
        Assertions.assertNotEquals(1, first)
        Assertions.assertTrue(f.hasNext())
        val second = f.nextInt()
        Assertions.assertNotEquals(first, second)
        Assertions.assertFalse(f.hasNext())
    }

    @Property
    fun degeneracyOrderingCoversConnectedVertices(
        @ForAll("arbitraryAdjacencyLikes") adjacencyLikes: List<Set<Int>>
    ): Boolean {
        val adjacencies = makeSymmetricAdjacencies(adjacencyLikes)
        val g = UndirectedGraph(adjacencies)
        val connectedVertices: SortedSet<Int> = g.connectedVertices(TreeSet())
        return sortedDegeneracyOrderingIncludingNeighbours(g) == connectedVertices
    }

    @Property
    fun degeneracyOrderingDropsSome(
        @ForAll("arbitraryAdjacencyLikes") adjacencyLikes: List<Set<Int>>
    ): Boolean {
        val adjacencies: List<Set<Int>> = makeSymmetricAdjacencies(adjacencyLikes)
        val g = UndirectedGraph(adjacencies)
        val connectedVertices = g.connectedVertices(HashSet())
        val filtered = DegeneracyFilter(g).stream().count()
        return if (connectedVertices.isEmpty()) {
            filtered == 0L
        } else {
            filtered < connectedVertices.size
        }
    }

    @Property
    fun degeneracyOrderingStartsWithLowestDegree(
        @ForAll("arbitraryAdjacencyLikes") adjacencyLikes: List<Set<Int>>
    ): Boolean {
        val adjacencies = makeSymmetricAdjacencies(adjacencyLikes)
        val g = UndirectedGraph(adjacencies)
        val f = DegeneracyFilter(g)
        return if (f.hasNext()) {
            val first = f.nextInt()
            f.stream().allMatch { v -> g.degree(first) <= g.degree(v) }
        } else {
            true
        }
    }

    private fun arbitraryNeighbours(order: Int): Arbitrary<Set<Int>> {
        return Arbitraries.integers().between(0, order - 1).set()
    }

    // Provide arbitrary input for makeSymmetricAdjacencies.
    @Suppress("unused")
    @Provide
    private fun arbitraryAdjacencyLikes(): Arbitrary<List<Set<Int>>> {
        val order: Arbitrary<Int> = Arbitraries.integers().between(1, 99)
        return order.flatMap { o: Int -> arbitraryNeighbours(o).list().ofSize(o) }
    }
}
