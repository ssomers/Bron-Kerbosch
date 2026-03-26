package be.steinsomers.bron_kerbosch

fun isLoopFree(adjacencies: List<Set<Int>>): Boolean {
    return adjacencies.asSequence().filterIndexed { i, neighbours -> neighbours.contains(i) }
        .none()
}

fun areNeighboursReciprocalIn(adjacencies: List<Set<Int>>, v: Int, neighbours: Set<Int>): Boolean {
    return neighbours.all { w -> adjacencies[w].contains(v) }
}

fun isSymmetric(adjacencies: List<Set<Int>>): Boolean {
    return adjacencies.foldIndexed(true) { index, valid, neighbours ->
        valid && areNeighboursReciprocalIn(adjacencies, index, neighbours)
    }
}

data class UndirectedGraph(private val adjacencies: List<Set<Int>>) {
    val size: Int
    val maxDegree: Int

    init {
        Debug.assert { isLoopFree(adjacencies) }
        Debug.assert { isSymmetric(adjacencies) }
        maxDegree = if (adjacencies.any()) adjacencies.maxOf(Set<Int>::size) else 0
        val totalDegree = adjacencies.sumOf(Set<Int>::size)
        assert(totalDegree % 2 == 0)
        size = totalDegree / 2
    }

    val order: Int
        get() {
            return adjacencies.size
        }

    fun degree(vertex: Int): Int {
        return adjacencies[vertex].size
    }

    fun hasDegree(vertex: Int): Boolean {
        return adjacencies[vertex].isNotEmpty()
    }

    fun neighbours(vertex: Int): Set<Int> {
        return adjacencies[vertex]
    }

    fun connectedVertices(): Sequence<Int> {
        return (0..<order).asSequence().filter(this::hasDegree)
    }

    fun maxDegreeVertices(): Sequence<Int> {
        return (0..<order).asSequence().filter { v -> degree(v) == maxDegree }
    }
}
