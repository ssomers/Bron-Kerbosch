package be.steinsomers.bron_kerbosch

import java.util.function.Consumer

internal object BronKerboschPivot {
    fun explore(graph: UndirectedGraph, cliqueConsumer: Consumer<IntArray>, pivotChoice: PivotChoice) {
        val pivot = graph.maxDegreeVertex()
        if (pivot != null) {
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            val excluded: MutableSet<Int> = HashSet(graph.order)
            for (v in 0..<graph.order) {
                val neighbours = graph.neighbours(v)
                if (!neighbours.contains(pivot)) {
                    val neighbouringExcluded = Util.intersect(neighbours, excluded)
                    if (neighbouringExcluded.size < neighbours.size) {
                        val neighbouringCandidates = (neighbours subtract neighbouringExcluded).toMutableSet()
                        visit(
                            graph = graph, cliqueConsumer = cliqueConsumer,
                            pivotChoice = pivotChoice,
                            candidates = neighbouringCandidates, excluded = neighbouringExcluded,
                            cliqueInProgress = intArrayOf(v)
                        )
                    }
                    excluded.add(v)
                }
            }
        }
    }

    fun visit(
        graph: UndirectedGraph,
        cliqueConsumer: Consumer<IntArray>,
        pivotChoice: PivotChoice,
        candidates: MutableSet<Int>,
        excluded: MutableSet<Int>,
        cliqueInProgress: IntArray
    ) {
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
                graph, cliqueConsumer, candidates, excluded, cliqueInProgress, PivotChoice.Arbitrary,
                pivot, remainingCandidates
            )
        } else {
            visitMaxDegree(graph, cliqueConsumer, candidates, excluded, cliqueInProgress, pivotChoice)
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
            val localDegree = Util.intersectionSize(neighbours, candidates)
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
                val localDegree = Util.intersectionSize(neighbours, candidates)
                if (seenLocalDegree < localDegree) {
                    seenLocalDegree = localDegree
                    pivot = v
                }
            }
        }
        visitAroundPivot(
            graph, cliqueConsumer, candidates, excluded, cliqueInProgress, pivotChoice, pivot, remainingCandidates
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
                        graph = graph, cliqueConsumer = cliqueConsumer,
                        pivotChoice = furtherPivotChoice,
                        candidates = neighbouringCandidates, excluded = neighbouringExcluded,
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
