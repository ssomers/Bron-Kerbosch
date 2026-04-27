package be.steinsomers.bron_kerbosch

fun isLoopFree(adjacencies: List<Set<Vertex>>): Boolean {
    return adjacencies.asSequence().filterIndexed { i, neighbours -> neighbours.contains(Vertex(i)) }
        .none()
}

fun areNeighboursReciprocalIn(adjacencies: List<Set<Vertex>>, v: Vertex, neighbours: Set<Vertex>): Boolean {
    return neighbours.all { w -> adjacencies[w.index].contains(v) }
}

fun isSymmetric(adjacencies: List<Set<Vertex>>): Boolean {
    return adjacencies.foldIndexed(true) { index, valid, neighbours ->
        valid && areNeighboursReciprocalIn(adjacencies, Vertex(index), neighbours)
    }
}

data class UndirectedGraph(private val adjacencies: List<Set<Vertex>>) {
    val size: Int
    val maxDegree: Int

    init {
        Debug.assert { isLoopFree(adjacencies) }
        Debug.assert { isSymmetric(adjacencies) }
        maxDegree = if (adjacencies.any()) adjacencies.maxOf(Set<Vertex>::size) else 0
        val totalDegree = adjacencies.sumOf(Set<Vertex>::size)
        assert(totalDegree % 2 == 0)
        size = totalDegree / 2
    }

    val order: Int
        get() {
            return adjacencies.size
        }

    fun degree(vertex: Vertex): Int {
        return adjacencies[vertex.index].size
    }

    fun hasDegree(vertex: Vertex): Boolean {
        return adjacencies[vertex.index].isNotEmpty()
    }

    fun neighbours(vertex: Vertex): Set<Vertex> {
        return adjacencies[vertex.index]
    }

    fun connectedVertices(): Sequence<Vertex> {
        return (0..<order).asSequence().map { i -> Vertex(i) }.filter(this::hasDegree)
    }

    fun maxDegreeVertices(): Sequence<Vertex> {
        return (0..<order).asSequence().map { i -> Vertex(i) }.filter { v -> degree(v) == maxDegree }
    }
}
