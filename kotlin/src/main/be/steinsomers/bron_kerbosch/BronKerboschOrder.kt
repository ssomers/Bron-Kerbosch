// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch

import java.util.stream.Stream

internal object BronKerboschOrder {
    fun explore(graph: UndirectedGraph, furtherPivotChoice: PivotChoice): Stream<IntArray> {
        val cliqueStream = Stream.builder<IntArray>()
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        val excluded: MutableSet<Int> = HashSet(graph.order)
        for (v in DegeneracyOrdering(graph, drop=1)) {
            val neighbours = graph.neighbours(v)
            require(neighbours.isNotEmpty())
            val neighbouringExcluded = Util.intersect(neighbours, excluded)
            if (neighbouringExcluded.size < neighbours.size) {
                val neighbouringCandidates = neighbours subtract neighbouringExcluded
                BronKerboschPivot.visit(
                    graph, cliqueStream,
                    pivotChoice=furtherPivotChoice,
                    candidates=neighbouringCandidates.toMutableSet(),
                    excluded=neighbouringExcluded.toMutableSet(),
                    cliqueInProgress=intArrayOf(v)
                )
            }
            excluded.add(v)
        }
        return cliqueStream.build()
    }
}
