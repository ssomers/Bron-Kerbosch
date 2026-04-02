package be.steinsomers.bron_kerbosch

class BronKerbosch2 : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        BronKerboschPivot.explore(graph, cliqueConsumer, PivotChoice.Arbitrary)
    }
}
