// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch

internal object BronKerboschDegeneracy {
    fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer, furtherPivotChoice: PivotChoice) {
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        val excluded = BooleanArray(graph.order)
        for (v in GraphDegeneracy(graph)) {
            val neighbours = graph.neighbours(v)
            require(neighbours.isNotEmpty())
            val neighbouringExcluded = Util.intersect(neighbours, excluded)
            if (neighbouringExcluded.size < neighbours.size) {
                val neighbouringCandidates = (neighbours subtract neighbouringExcluded).toMutableSet()
                BronKerboschPivot.visit(
                    graph = graph, cliqueConsumer = cliqueConsumer,
                    pivotChoice = furtherPivotChoice,
                    candidates = neighbouringCandidates,
                    excluded = neighbouringExcluded,
                    cliqueInProgress = intArrayOf(v)
                )
            }
            excluded[v] = true
        }
    }
}
