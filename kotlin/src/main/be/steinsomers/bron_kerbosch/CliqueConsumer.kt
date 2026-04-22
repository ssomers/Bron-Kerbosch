package be.steinsomers.bron_kerbosch

import java.util.Collections
import kotlin.concurrent.atomics.AtomicInt
import kotlin.concurrent.atomics.ExperimentalAtomicApi
import kotlin.math.min

abstract class CliqueStorage {
    abstract fun store(clique: Clique)
}

class CliqueCollector : CliqueStorage() {
    private val cliques = Collections.synchronizedCollection(mutableListOf<Clique>())

    override fun store(clique: Clique) {
        cliques.add(clique)
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

@OptIn(ExperimentalAtomicApi::class)
class CliqueCounter : CliqueStorage() {
    private val cliques = AtomicInt(0)

    override fun store(clique: Clique) {
        cliques.addAndFetch(1)
    }

    fun harvest(): Int {
        return cliques.load()
    }
}

data class CliqueConsumer(val minSize: Int, val storage: CliqueStorage) {
    fun accept(clique: Clique) {
        storage.store(clique)
    }
}
