package be.steinsomers.bron_kerbosch

class BronKerbosch3gp : BronKerboschAlgorithm {
    override val name: String = "Ver3½-GP"
    override val hasRaceCondition: Boolean = false

    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        BronKerboschDegeneracy.explore(graph, cliqueConsumer, PivotChoice.MaxDegreeLocal)
    }
}
