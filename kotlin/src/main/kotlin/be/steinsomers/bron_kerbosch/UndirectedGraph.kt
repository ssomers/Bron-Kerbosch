package be.steinsomers.bron_kerbosch

import java.util.Collections
import java.util.stream.IntStream
import java.util.stream.Stream

data class UndirectedGraph(private val adjacencies: List<Set<Int>>) {
    init {
        Debug.assert { adjacencies.indices.none { v -> adjacencies[v].contains(v) } }
        Debug.assert { adjacencies.indices.all { v -> adjacencies[v].all { w -> adjacencies[w].contains(v) } } }
    }

    val order: Int
        get() {
            return adjacencies.size
        }

    fun size(): Int {
        val total = adjacencies.sumOf(Set<Int>::size)
        require(total % 2 == 0)
        return total / 2
    }

    fun degree(node: Int): Int {
        return adjacencies[node].size
    }

    fun hasDegree(node: Int): Boolean {
        return adjacencies[node].isNotEmpty()
    }

    fun neighbours(node: Int): Set<Int> {
        return Collections.unmodifiableSet(adjacencies[node])
    }

    fun connectedVertices(): Stream<Int> {
        return IntStream.range(0, order).filter(::hasDegree).boxed()
    }

    fun maxDegreeVertex(): Int {
        return IntStream.range(0, order).boxed()
            .max(Comparator.comparingInt(this::degree))
            .orElseThrow()
    }
}
