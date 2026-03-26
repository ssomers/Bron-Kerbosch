// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch

internal object BronKerboschDegeneracy {
    fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer, furtherPivotChoice: PivotChoice) {
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        val excluded = BooleanArray(graph.order)
        GraphDegeneracy(graph).forEach { v ->
            val neighbours = graph.neighbours(v)
            Debug.assert { neighbours.isNotEmpty() }
            val neighbouringExcluded = neighbours.filterTo(HashSet()) { v -> excluded[v] }
            if (neighbouringExcluded.size < neighbours.size) {
                val neighbouringCandidates = neighbours.subtract(neighbouringExcluded).toMutableSet()
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
