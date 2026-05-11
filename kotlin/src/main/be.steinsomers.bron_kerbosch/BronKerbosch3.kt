package be.steinsomers.bron_kerbosch

class BronKerbosch3 : BronKerboschAlgorithm {
    override val name: String = "Ver3½"

    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        BronKerboschDegeneracy.explore(graph, cliqueConsumer, PivotChoice.Arbitrary)
    }
}
