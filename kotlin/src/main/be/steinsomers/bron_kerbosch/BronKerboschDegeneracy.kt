// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch

internal object BronKerboschDegeneracy {
    fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer, furtherPivotChoice: PivotChoice) {
        val degeneracy = GraphDegeneracy(graph)
        degeneracy.forEach { v ->
            val (neighbouringCandidates, neighbouringExcluded) =
                Util.partition(graph.neighbours(v)) { v -> degeneracy.isCandidate(v) }
            Debug.assert { neighbouringCandidates.isNotEmpty() }
            BronKerboschPivot.visit(
                graph = graph, cliqueConsumer = cliqueConsumer,
                pivotChoice = furtherPivotChoice,
                candidates = neighbouringCandidates,
                excluded = neighbouringExcluded,
                clique = Clique.singleton(v)
            )
        }
    }
}
