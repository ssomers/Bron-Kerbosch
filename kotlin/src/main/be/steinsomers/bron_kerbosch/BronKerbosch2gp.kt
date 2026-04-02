package be.steinsomers.bron_kerbosch

class BronKerbosch2gp : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        BronKerboschPivot.explore(graph, cliqueConsumer, PivotChoice.MaxDegreeLocal)
    }
}
