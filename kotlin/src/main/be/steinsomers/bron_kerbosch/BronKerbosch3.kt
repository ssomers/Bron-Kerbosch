package be.steinsomers.bron_kerbosch

class BronKerbosch3 : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: (IntArray) -> Unit) {
        BronKerboschOrder.explore(graph, cliqueConsumer, PivotChoice.Arbitrary)
    }
}
