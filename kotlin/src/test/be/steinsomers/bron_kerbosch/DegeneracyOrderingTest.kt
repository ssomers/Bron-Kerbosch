package be.steinsomers.bron_kerbosch

import net.jqwik.api.*
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test
import java.util.Arrays
import java.util.SortedSet
import java.util.TreeSet
import java.util.function.IntConsumer
import java.util.stream.Stream
import kotlin.math.max

internal class DegeneracyOrderingTest {
    private fun sortedDegeneracyOrdering(g: UndirectedGraph, drop: Int): SortedSet<Int> {
        val vertices: SortedSet<Int> = TreeSet()
        DegeneracyOrdering(g, drop=drop).forEachRemaining(IntConsumer { v: Int ->
            val added = vertices.add(v)
            Assertions.assertTrue(added)
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
        Assertions.assertTrue(sortedDegeneracyOrdering(g, drop=0).isEmpty())
        Assertions.assertTrue(sortedDegeneracyOrdering(g, drop=1).isEmpty())
    }

    @Test
    fun pair() {
        val g = UndirectedGraph(listOf(setOf(1), setOf(0)))
        Assertions.assertEquals(setOf(0, 1), sortedDegeneracyOrdering(g, drop=0))
        Assertions.assertEquals(1, sortedDegeneracyOrdering(g, drop=1).size)
        Assertions.assertEquals(0, sortedDegeneracyOrdering(g, drop=2).size)
    }

    @Test
    fun split() {
        val g = UndirectedGraph(listOf(setOf(1), setOf(0, 2), setOf(1)))
        Assertions.assertNotEquals(1, DegeneracyOrdering(g, drop=0).next())
        Assertions.assertEquals(setOf(0, 1, 2), sortedDegeneracyOrdering(g, drop=0))
    }

    @Property
    fun degeneracyOrderingCoversConnectedVertices(
        @ForAll("arbitraryAdjacencyLikes") adjacencyLikes: List<Set<Int>>
    ): Boolean {
        val adjacencies = makeSymmetricAdjacencies(adjacencyLikes)
        val g = UndirectedGraph(adjacencies)
        val connectedVertices = g.connectedVertices().toSortedSet()
        return sortedDegeneracyOrdering(g, drop=0) == connectedVertices
    }

    @Property
    fun degeneracyOrderingDrops1(
        @ForAll("arbitraryAdjacencyLikes") adjacencyLikes: List<Set<Int>>
    ): Boolean {
        val adjacencies: List<Set<Int>> = makeSymmetricAdjacencies(adjacencyLikes)
        val g = UndirectedGraph(adjacencies)
        val ordering = DegeneracyOrdering(g, drop=0).stream().toArray()
        val ordering1 = DegeneracyOrdering(g, drop=1).stream().toArray()
        return ordering1.size == max(0, ordering.size - 1)
                && Arrays.equals(ordering, 0, ordering1.size, ordering1, 0, ordering1.size)
    }

    @Property
    fun degeneracyOrderingStartsWithLowestDegree(
        @ForAll("arbitraryAdjacencyLikes") adjacencyLikes: List<Set<Int>>
    ): Boolean {
        val adjacencies = makeSymmetricAdjacencies(adjacencyLikes)
        val g = UndirectedGraph(adjacencies)
        val ordering = DegeneracyOrdering(g, drop=0)
        val first: Int
        try {
            first = ordering.nextInt()
        } catch (_: NoSuchElementException) {
            return true
        }
        return ordering.stream().allMatch { v -> g.degree(first) <= g.degree(v) }
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
