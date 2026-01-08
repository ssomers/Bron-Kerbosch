package be.steinsomers.bron_kerbosch

data class UndirectedGraph(private val adjacencies: List<Set<Int>>) {
    init {
        Debug.assert { adjacencies.indices.none { v -> adjacencies[v].contains(v) } }
        Debug.assert { adjacencies.indices.all { v -> adjacencies[v].all { w -> adjacencies[w].contains(v) } } }
        Debug.assert { adjacencies.sumOf(Set<Int>::size) % 2 == 0 }
    }

    val order: Int
        get() {
            return adjacencies.size
        }

    fun size(): Int {
        return adjacencies.sumOf(Set<Int>::size) / 2
    }

    fun degree(node: Int): Int {
        return adjacencies[node].size
    }

    fun hasDegree(node: Int): Boolean {
        return adjacencies[node].isNotEmpty()
    }

    fun neighbours(node: Int): Set<Int> {
        return adjacencies[node]
    }

    fun <C : MutableCollection<Int>> connectedVertices(destination: C): C {
        return (0..<order).filterTo(destination, this::hasDegree)
    }

    fun maxDegreeVertex(): Int? {
        return (0..<order).maxByOrNull(this::degree)
    }
}
