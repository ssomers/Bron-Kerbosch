package be.steinsomers.bron_kerbosch

import java.util.stream.Stream

class BronKerbosch3 : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph): Stream<IntArray> {
        return BronKerboschOrder.explore(graph, PivotChoice.Arbitrary)
    }
}
