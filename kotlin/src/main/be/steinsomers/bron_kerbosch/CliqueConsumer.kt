package be.steinsomers.bron_kerbosch

import kotlin.math.min

abstract class CliqueStorage {
    abstract fun isEmpty(): Boolean
    abstract fun store(clique: Clique)
    abstract fun spawn(n: Int): List<CliqueStorage>
    abstract fun absorb(spawned: List<CliqueStorage>)
}

class CliqueCollector : CliqueStorage() {
    private val cliques = mutableListOf<Clique>()

    override fun isEmpty(): Boolean {
        return cliques.isEmpty()
    }

    override fun store(clique: Clique) {
        cliques.add(clique)
    }

    override fun spawn(n: Int): List<CliqueStorage> {
        require(isEmpty())
        return List(n) { CliqueCollector() }
    }

    @Suppress("UNCHECKED_CAST")
    override fun absorb(spawned: List<CliqueStorage>) {
        require(isEmpty())
        val spawned: List<CliqueCollector> = spawned as List<CliqueCollector>
        spawned.forEach { s -> cliques.addAll(s.cliques) }
    }

    fun toSortedList(): List<SortedClique> {
        return cliques
            .map { clique -> SortedClique.freeze(clique) }
            .sortedWith { clique1: SortedClique, clique2: SortedClique ->
                when (val diff = (0..<min(clique1.size(), clique2.size())).asSequence()
                    .map { i -> clique1.vertices[i] - clique2.vertices[i] }
                    .firstOrNull { diff -> diff != 0 }) {
                    null -> throw IllegalArgumentException("got overlapping or equal cliques $clique1 <> $clique2")
                    else -> diff
                }
            }
    }
}

class CliqueCounter : CliqueStorage() {
    private var cliques: Int = 0

    override fun isEmpty(): Boolean {
        return cliques == 0
    }

    override fun store(clique: Clique) {
        cliques += 1
    }

    override fun spawn(n: Int): List<CliqueStorage> {
        require(isEmpty())
        return List(n) { CliqueCounter() }
    }

    @Suppress("UNCHECKED_CAST")
    override fun absorb(spawned: List<CliqueStorage>) {
        require(isEmpty())
        val spawned: List<CliqueCounter> = spawned as List<CliqueCounter>
        cliques = spawned.sumOf(CliqueCounter::cliques)
    }

    fun harvest(): Int {
        return cliques
    }
}

data class CliqueConsumer(val minSize: Int, val storage: CliqueStorage) {
    fun accept(clique: Clique) {
        storage.store(clique)
    }
}
