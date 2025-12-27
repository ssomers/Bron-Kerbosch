package be.steinsomers.bron_kerbosch

import java.util.function.Consumer
import java.util.stream.Stream

internal object BronKerboschPivot {
    fun explore(graph: UndirectedGraph, pivotChoice: PivotChoice): Stream<IntArray> {
        val cliqueStream = Stream.builder<IntArray>()
        if (graph.order > 0) {
            val pivot = graph.maxDegreeVertex()
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            val excluded: MutableSet<Int> = HashSet(graph.order)
            for (v in 0..<graph.order) {
                val neighbours = graph.neighbours(v)
                if (!neighbours.contains(pivot)) {
                    val neighbouringExcluded = neighbours intersect excluded
                    if (neighbouringExcluded.size < neighbours.size) {
                        val neighbouringCandidates = neighbours subtract neighbouringExcluded
                        visit(
                            graph, cliqueStream, pivotChoice = pivotChoice,
                            candidates = neighbouringCandidates.toMutableSet(),
                            excluded = neighbouringExcluded.toMutableSet(),
                            cliqueInProgress = intArrayOf(v)
                        )
                    }
                    excluded.add(v)
                }
            }
        }
        return cliqueStream.build()
    }

    fun visit(
        graph: UndirectedGraph,
        cliqueConsumer: Consumer<IntArray>,
        pivotChoice: PivotChoice,
        candidates: MutableSet<Int>,
        excluded: MutableSet<Int>,
        cliqueInProgress: IntArray
    ) {
        @Suppress("DuplicatedCode")
        Debug.assert { candidates.all(graph::hasDegree) }
        Debug.assert { excluded.all(graph::hasDegree) }
        Debug.assert { Util.areDisjoint(candidates, excluded) }
        Debug.assert { candidates.isNotEmpty() }
        if (candidates.size == 1) {
            // Same logic as below, stripped down for this common case
            val v = candidates.iterator().next()
            val neighbours = graph.neighbours(v)
            if (Util.areDisjoint(neighbours, excluded)) {
                cliqueConsumer.accept(Util.append(cliqueInProgress, v))
            }
        } else if (pivotChoice == PivotChoice.Arbitrary) {
            val remainingCandidates = ArrayList<Int>(candidates)
            val pivot = remainingCandidates[0]
            visitAroundPivot(
                graph, cliqueConsumer, candidates, excluded, cliqueInProgress,
                PivotChoice.Arbitrary, pivot, remainingCandidates
            )
        } else {
            visitMaxDegree(
                graph, cliqueConsumer, candidates, excluded, cliqueInProgress,
                pivotChoice
            )
        }
    }

    private fun visitMaxDegree(
        graph: UndirectedGraph, cliqueConsumer: Consumer<IntArray>,
        candidates: MutableSet<Int>, excluded: MutableSet<Int>,
        cliqueInProgress: IntArray, pivotChoice: PivotChoice
    ) {
        require(pivotChoice == PivotChoice.MaxDegreeLocal || pivotChoice == PivotChoice.MaxDegreeLocalX)
        // Quickly handle locally unconnected candidates while finding pivot
        var pivot = -1
        val remainingCandidates: MutableCollection<Int> = ArrayList(candidates.size)
        var seenLocalDegree = 0
        for (v in candidates) {
            val neighbours = graph.neighbours(v)
            val localDegree = Util.intersect(neighbours, candidates).size
            if (localDegree == 0) {
                // Same logic as below, stripped down
                if (Util.areDisjoint(neighbours, excluded)) {
                    cliqueConsumer.accept(Util.append(cliqueInProgress, v))
                }
            } else {
                if (seenLocalDegree < localDegree) {
                    seenLocalDegree = localDegree
                    pivot = v
                }
                remainingCandidates.add(v)
            }
        }
        if (pivotChoice == PivotChoice.MaxDegreeLocalX && remainingCandidates.isNotEmpty()) {
            for (v in excluded) {
                val neighbours = graph.neighbours(v)
                val localDegree = Util.intersect(neighbours, candidates).size
                if (seenLocalDegree < localDegree) {
                    seenLocalDegree = localDegree
                    pivot = v
                }
            }
        }
        visitAroundPivot(
            graph, cliqueConsumer, candidates, excluded, cliqueInProgress,
            pivotChoice, pivot, remainingCandidates
        )
    }

    private fun visitAroundPivot(
        graph: UndirectedGraph, cliqueConsumer: Consumer<IntArray>,
        candidates: MutableSet<Int>, excluded: MutableSet<Int>,
        cliqueInProgress: IntArray, furtherPivotChoice: PivotChoice,
        pivot: Int, remainingCandidates: Iterable<Int>
    ) {
        for (v in remainingCandidates) {
            val neighbours = graph.neighbours(v)
            if (!neighbours.contains(pivot)) {
                candidates.remove(v)
                val neighbouringCandidates = Util.intersect(neighbours, candidates)
                if (neighbouringCandidates.isNotEmpty()) {
                    val neighbouringExcluded = Util.intersect(neighbours, excluded)
                    visit(
                        graph, cliqueConsumer,
                        pivotChoice = furtherPivotChoice,
                        candidates = neighbouringCandidates.toMutableSet(),
                        excluded = neighbouringExcluded.toMutableSet(),
                        cliqueInProgress = Util.append(cliqueInProgress, v)
                    )
                } else if (Util.areDisjoint(neighbours, excluded)) {
                    cliqueConsumer.accept(Util.append(cliqueInProgress, v))
                }
                excluded.add(v)
            }
        }
    }
}
