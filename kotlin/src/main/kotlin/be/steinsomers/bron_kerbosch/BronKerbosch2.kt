package be.steinsomers.bron_kerbosch

import java.util.stream.Stream

class BronKerbosch2 : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph): Stream<IntArray> {
        return BronKerboschPivot.explore(graph, PivotChoice.Arbitrary)
    }
}
