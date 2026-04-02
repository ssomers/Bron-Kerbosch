package be.steinsomers.bron_kerbosch

fun interface BronKerboschAlgorithm {
    @Throws(InterruptedException::class)
    fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer)
}

data class CliqueConsumer(val minSize: Int, private val acceptor: (IntArray) -> Unit) {
    fun accept(clique: IntArray) {
        acceptor.invoke(clique)
    }
}
