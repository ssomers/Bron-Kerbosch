// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch

import java.util.stream.Stream

internal object BronKerboschOrder {
    fun explore(graph: UndirectedGraph, furtherPivotChoice: PivotChoice): Stream<IntArray> {
        val cliqueStream = Stream.builder<IntArray>()
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        val excluded: MutableSet<Int> = HashSet(graph.order)
        val vertices = Iterable { DegeneracyOrdering(graph, -1) }
        for (v in vertices) {
            val neighbours = graph.neighbours(v)
            require(neighbours.isNotEmpty())
            val neighbouringExcluded = Util.intersect(neighbours, excluded)
            if (neighbouringExcluded.size < neighbours.size) {
                val neighbouringCandidates = neighbours subtract neighbouringExcluded
                BronKerboschPivot.visit(
                    graph, cliqueStream,
                    furtherPivotChoice,
                    neighbouringCandidates.toMutableSet(),
                    neighbouringExcluded.toMutableSet(),
                    intArrayOf(v)
                )
            }
            excluded.add(v)
        }
        return cliqueStream.build()
    }
}
