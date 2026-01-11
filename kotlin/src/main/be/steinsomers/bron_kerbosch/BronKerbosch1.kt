// Naive Bron-Kerbosch algorithm
package be.steinsomers.bron_kerbosch

import java.util.function.Consumer

class BronKerbosch1 : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: (IntArray) -> Unit) {
        val candidates: MutableSet<Int> = graph.connectedVertices(HashSet())
        val excluded: MutableSet<Int> = HashSet(candidates.size)
        visit(
            graph = graph, cliqueConsumer = cliqueConsumer,
            candidates = candidates, excluded = excluded,
            cliqueInProgress = intArrayOf()
        )
    }

    companion object {
        private fun visit(
            graph: UndirectedGraph, cliqueConsumer: Consumer<IntArray>,
            candidates: MutableSet<Int>, excluded: MutableSet<Int>,
            cliqueInProgress: IntArray
        ) {
            Debug.assert { candidates.all(graph::hasDegree) }
            Debug.assert { excluded.all(graph::hasDegree) }
            Debug.assert { Util.areDisjoint(candidates, excluded) }
            while (candidates.isNotEmpty()) {
                val v = Util.popArbitrary(candidates)
                val neighbours = graph.neighbours(v)
                val neighbouringCandidates = Util.intersect(candidates, neighbours)
                if (neighbouringCandidates.isNotEmpty()) {
                    val neighbouringExcluded = Util.intersect(excluded, neighbours)
                    visit(
                        graph = graph, cliqueConsumer = cliqueConsumer,
                        candidates = neighbouringCandidates, excluded = neighbouringExcluded,
                        cliqueInProgress = Util.append(cliqueInProgress, v)
                    )
                } else if (Util.areDisjoint(excluded, neighbours)) {
                    cliqueConsumer.accept(Util.append(cliqueInProgress, v))
                }
                excluded.add(v)
            }
        }
    }
}
