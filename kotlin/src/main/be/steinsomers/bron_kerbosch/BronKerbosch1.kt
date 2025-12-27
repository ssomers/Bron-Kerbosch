// Naive Bron-Kerbosch algorithm
package be.steinsomers.bron_kerbosch

import java.util.function.Consumer
import java.util.stream.Collectors
import java.util.stream.Stream

class BronKerbosch1 : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph): Stream<IntArray> {
        val cliqueStream = Stream.builder<IntArray>()
        val candidates: MutableSet<Int> = graph.connectedVertices().collect(Collectors.toSet())
        if (candidates.isNotEmpty()) {
            val excluded: MutableSet<Int> = HashSet(candidates.size)
            visit(graph, cliqueStream, candidates, excluded, BronKerboschAlgorithm.EMPTY_CLIQUE)
        }
        return cliqueStream.build()
    }

    companion object {
        private fun visit(
            graph: UndirectedGraph, cliqueConsumer: Consumer<IntArray>,
            candidates: MutableSet<Int>, excluded: MutableSet<Int>,
            cliqueInProgress: IntArray
        ) {
            @Suppress("DuplicatedCode")
            Debug.assert { candidates.all(graph::hasDegree) }
            Debug.assert { excluded.all(graph::hasDegree) }
            Debug.assert { Util.areDisjoint(candidates, excluded) }
            Debug.assert { candidates.isNotEmpty() }
            while (candidates.isNotEmpty()) {
                val v = Util.popArbitrary(candidates)
                val neighbours = graph.neighbours(v)
                val neighbouringCandidates = Util.intersect(candidates, neighbours)
                if (neighbouringCandidates.isNotEmpty()) {
                    val neighbouringExcluded = Util.intersect(excluded, neighbours)
                    visit(
                        graph, cliqueConsumer,
                        neighbouringCandidates.toMutableSet(),
                        neighbouringExcluded.toMutableSet(),
                        Util.append(cliqueInProgress, v)
                    )
                } else if (Util.areDisjoint(excluded, neighbours)) {
                    cliqueConsumer.accept(Util.append(cliqueInProgress, v))
                }
                excluded.add(v)
            }
        }
    }
}
