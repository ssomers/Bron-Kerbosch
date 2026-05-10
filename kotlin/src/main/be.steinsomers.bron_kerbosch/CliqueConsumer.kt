package be.steinsomers.bron_kerbosch

import kotlin.math.min

abstract class CliqueStorage {
    abstract fun store(clique: Clique)
    abstract fun spawn(n: Int): List<CliqueStorage>
    abstract fun absorb(spawned: List<CliqueStorage>)
}

class CliqueCollector : CliqueStorage() {
    private val cliques = mutableListOf<Clique>()

    override fun store(clique: Clique) {
        cliques.add(clique)
    }

    override fun spawn(n: Int): List<CliqueStorage> {
        require(cliques.isEmpty())
        return List(n) { CliqueCollector() }
    }

    @Suppress("UNCHECKED_CAST")
    override fun absorb(spawned: List<CliqueStorage>) {
        require(cliques.isEmpty())
        val spawned = spawned as List<CliqueCollector>
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

    @Suppress("unused", "RedundantSuppression")
    override fun store(clique: Clique) {
        cliques += 1
    }

    override fun spawn(n: Int): List<CliqueStorage> {
        require(cliques == 0)
        return List(n) { CliqueCounter() }
    }

    @Suppress("UNCHECKED_CAST")
    override fun absorb(spawned: List<CliqueStorage>) {
        require(cliques == 0)
        val spawned = spawned as List<CliqueCounter>
        cliques = spawned.sumOf(CliqueCounter::cliques)
    }

    fun harvest(): Int {
        return cliques
    }
}

data class CliqueConsumer(val minSize: Int, val storage: CliqueStorage) {
    init {
        require(minSize >= 2) // we don't want to write code for the trivial 0-clique or 1-cliques
    }

    fun accept(clique: Clique) {
        Debug.assert { clique.size() >= minSize }
        storage.store(clique)
    }
}
