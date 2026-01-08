package be.steinsomers.bron_kerbosch

class BronKerbosch2 : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: (IntArray) -> Unit) {
        BronKerboschPivot.explore(graph, cliqueConsumer, PivotChoice.Arbitrary)
    }
}
