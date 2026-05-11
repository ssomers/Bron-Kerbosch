package be.steinsomers.bron_kerbosch

class BronKerbosch3gpx : BronKerboschAlgorithm {
    override val name: String = "Ver3½-GPX"

    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        BronKerboschDegeneracy.explore(graph, cliqueConsumer, PivotChoice.MaxDegreeLocalX)
    }
}
