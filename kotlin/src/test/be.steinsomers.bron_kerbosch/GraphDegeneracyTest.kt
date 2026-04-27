package be.steinsomers.bron_kerbosch

import net.jqwik.api.*
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test
import java.util.*

fun graph(adjacencies: List<Set<Int>>): UndirectedGraph {
    return UndirectedGraph(adjacencies.map { neighbours -> neighbours.mapTo(TreeSet()) { i -> Vertex(i) } })
}


internal class GraphDegeneracyTest {
    private fun sortedDegeneracyOrderingIncludingNeighbours(g: UndirectedGraph): SortedSet<Vertex> {
        val vertices: SortedSet<Vertex> = TreeSet()
        GraphDegeneracy(g).forEachRemaining { v ->
            vertices.add(v)
            vertices.addAll(g.neighbours(v))
        }
        return vertices
    }

    /**
     * @param adjacencyLikes List of suggested neighbours, where numbers ≥ index are offset by 1.
     *                       Omits the last vertex because we make the actual adjacencies symmetric.
     */
    private fun makeSymmetricAdjacencies(adjacencyLikes: List<Set<Int>>): List<Set<Int>> {
        val order = adjacencyLikes.size + 1
        val adjacencies = List(order) { HashSet<Int>() }
        adjacencyLikes.forEachIndexed { v, neighbourLikes ->
            for (w1 in neighbourLikes) {
                require(w1 < order - 1)
                val w = if (w1 >= v) w1 + 1 else w1
                adjacencies[v].add(w)
                adjacencies[w].add(v)
            }
        }
        return adjacencies
    }

    @Test
    fun empty() {
        val g = UndirectedGraph(listOf())
        Assertions.assertFalse(GraphDegeneracy(g).hasNext())
    }

    @Test
    fun single() {
        val g = graph(listOf(setOf()))
        Assertions.assertFalse(GraphDegeneracy(g).hasNext())
    }

    @Test
    fun pair() {
        val g = graph(listOf(setOf(1), setOf(0)))
        val f = GraphDegeneracy(g)
        Assertions.assertTrue(f.hasNext())
        f.next()
        Assertions.assertFalse(f.hasNext())
    }

    @Test
    fun split() {
        val g = graph(listOf(setOf(1), setOf(0, 2), setOf(1)))
        val f = GraphDegeneracy(g)
        Assertions.assertTrue(f.hasNext())
        val first = f.next()
        Assertions.assertNotEquals(Vertex(1), first)
        Assertions.assertTrue(f.hasNext())
        val second = f.next()
        Assertions.assertNotEquals(first, second)
        Assertions.assertFalse(f.hasNext())
    }

    @Property
    fun degeneracyOrderingCoversConnectedVertices(
        @ForAll("arbitraryAdjacencyLikes") adjacencyLikes: List<Set<Int>>
    ): Boolean {
        val adjacencies = makeSymmetricAdjacencies(adjacencyLikes)
        val g = graph(adjacencies)
        val connectedVertices: SortedSet<Vertex> = g.connectedVertices().toCollection(TreeSet())
        return sortedDegeneracyOrderingIncludingNeighbours(g) == connectedVertices
    }

    @Property
    fun degeneracyOrderingDropsSome(
        @ForAll("arbitraryAdjacencyLikes") adjacencyLikes: List<Set<Int>>
    ): Boolean {
        val adjacencies: List<Set<Int>> = makeSymmetricAdjacencies(adjacencyLikes)
        val g = graph(adjacencies)
        val connected = g.connectedVertices().count()
        val filtered = GraphDegeneracy(g).asSequence().count()
        return filtered < connected || (connected == 0 && filtered == 0)
    }

    @Property
    fun degeneracyOrderingStartsWithLowestDegree(
        @ForAll("arbitraryAdjacencyLikes") adjacencyLikes: List<Set<Int>>
    ): Boolean {
        val adjacencies = makeSymmetricAdjacencies(adjacencyLikes)
        val g = graph(adjacencies)
        val ordering = GraphDegeneracy(g)
        return if (ordering.hasNext()) {
            val first = ordering.next()
            ordering.asSequence().all { v -> g.degree(first) <= g.degree(v) }
        } else {
            true
        }
    }

    // Provide arbitrary input for makeSymmetricAdjacencies.
    @Suppress("unused")
    @Provide
    private fun arbitraryAdjacencyLikes(): Arbitrary<List<Set<Int>>> {
        val order: Arbitrary<Int> = Arbitraries.integers().between(2, 12)
        return order.flatMap { o: Int -> arbitraryNeighbourLikes(o).list().ofSize(o) }
    }

    private fun arbitraryNeighbourLikes(order: Int): Arbitrary<Set<Int>> {
        return Arbitraries.integers().between(0, order - 2).set()
    }
}
