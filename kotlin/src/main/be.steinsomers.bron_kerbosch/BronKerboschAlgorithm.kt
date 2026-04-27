package be.steinsomers.bron_kerbosch

interface BronKerboschAlgorithm {
    val name: String
    val hasRaceCondition: Boolean

    @Throws(InterruptedException::class)
    fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer)
}
