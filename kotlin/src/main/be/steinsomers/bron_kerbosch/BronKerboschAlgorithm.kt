package be.steinsomers.bron_kerbosch

fun interface BronKerboschAlgorithm {
    @Throws(InterruptedException::class)
    fun explore(graph: UndirectedGraph, cliqueConsumer: (IntArray) -> Unit)
}
