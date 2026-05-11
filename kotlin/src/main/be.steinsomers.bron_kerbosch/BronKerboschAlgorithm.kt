package be.steinsomers.bron_kerbosch

@Suppress("SameReturnValue")
interface BronKerboschAlgorithm {
    val name: String
    val deterministic: Boolean
        get() = true

    @Throws(InterruptedException::class)
    fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer)
}
