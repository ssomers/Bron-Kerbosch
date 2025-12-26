package be.steinsomers.bron_kerbosch

import java.util.stream.Stream

fun interface BronKerboschAlgorithm {
    @Throws(InterruptedException::class)
    fun explore(graph: UndirectedGraph): Stream<IntArray>

    companion object {
        val EMPTY_CLIQUE: IntArray = intArrayOf()
    }
}
