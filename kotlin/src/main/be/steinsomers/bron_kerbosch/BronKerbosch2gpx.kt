package be.steinsomers.bron_kerbosch

class BronKerbosch2gpx : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: (IntArray) -> Unit) {
        BronKerboschPivot.explore(graph, cliqueConsumer, PivotChoice.MaxDegreeLocalX)
    }
}
