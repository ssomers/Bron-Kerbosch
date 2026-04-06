// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch

internal object BronKerboschDegeneracy {
    fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer, furtherPivotChoice: PivotChoice) {
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        GraphDegeneracy(graph).forEach { item ->
            val v = item.pick
            val neighbouringExcluded = item.pickedNeighbours
            val neighbours = graph.neighbours(v)
            Debug.assert { neighbours.isNotEmpty() }
            if (neighbouringExcluded.size < neighbours.size) {
                val neighbouringCandidates = neighbours.subtract(neighbouringExcluded).toMutableSet()
                BronKerboschPivot.visit(
                    graph = graph, cliqueConsumer = cliqueConsumer,
                    pivotChoice = furtherPivotChoice,
                    candidates = neighbouringCandidates,
                    excluded = neighbouringExcluded,
                    cliqueInProgress = CliqueInProgress.singleton(v)
                )
            }
        }
    }
}
