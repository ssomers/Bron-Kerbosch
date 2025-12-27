package be.steinsomers.bron_kerbosch

import java.util.stream.Stream

class BronKerbosch3gp : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph): Stream<IntArray> {
        return BronKerboschOrder.explore(graph, PivotChoice.MaxDegreeLocal)
    }
}
