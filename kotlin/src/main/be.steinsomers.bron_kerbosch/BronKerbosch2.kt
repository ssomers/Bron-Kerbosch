package be.steinsomers.bron_kerbosch

class BronKerbosch2 : BronKerboschAlgorithm {
    override val name: String = "Ver2½"
    override val hasRaceCondition: Boolean = false

    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        BronKerboschPivot.explore(graph, cliqueConsumer, PivotChoice.Arbitrary)
    }
}
