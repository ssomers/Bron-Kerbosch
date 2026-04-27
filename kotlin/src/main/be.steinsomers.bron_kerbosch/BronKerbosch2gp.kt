package be.steinsomers.bron_kerbosch

class BronKerbosch2gp : BronKerboschAlgorithm {
    override val name: String = "Ver2½-GP"
    override val hasRaceCondition: Boolean = false

    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        BronKerboschPivot.explore(graph, cliqueConsumer, PivotChoice.MaxDegreeLocal)
    }
}
