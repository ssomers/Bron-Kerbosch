package be.steinsomers.bron_kerbosch

class BronKerbosch2 : BronKerboschAlgorithm {
    override val name: String = "Ver2½"

    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        BronKerboschPivot.explore(graph, cliqueConsumer, PivotChoice.Arbitrary)
    }
}
