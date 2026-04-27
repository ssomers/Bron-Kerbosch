package be.steinsomers.bron_kerbosch

internal object BronKerboschPivot {
    fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer, pivotChoice: PivotChoice) {
        val pivot = graph.maxDegreeVertices().firstOrNull()
        if (pivot != null) {
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            val excluded = BooleanArray(graph.order)
            graph.connectedVertices().forEach { v ->
                val neighbours = graph.neighbours(v)
                if (!neighbours.contains(pivot)) {
                    val neighbouringExcluded = neighbours.filterTo(HashSet()) { v -> excluded[v.index] }
                    if (neighbouringExcluded.size < neighbours.size) {
                        val neighbouringCandidates = (neighbours subtract neighbouringExcluded).toMutableSet()
                        visit(
                            graph = graph, cliqueConsumer = cliqueConsumer,
                            pivotChoice = pivotChoice,
                            candidates = neighbouringCandidates, excluded = neighbouringExcluded,
                            clique = Clique.singleton(v)
                        )
                    }
                    excluded[v.index] = true
                }
            }
        }
    }

    fun visit(
        graph: UndirectedGraph,
        cliqueConsumer: CliqueConsumer,
        pivotChoice: PivotChoice,
        candidates: MutableSet<Vertex>,
        excluded: MutableSet<Vertex>,
        clique: Clique
    ) {
        Debug.assert { candidates.all(graph::hasDegree) }
        Debug.assert { excluded.all(graph::hasDegree) }
        Debug.assert { Util.areDisjoint(candidates, excluded) }
        Debug.assert { candidates.isNotEmpty() }
        if (candidates.size == 1) {
            // Same logic as below, stripped down for this common case
            val v = candidates.iterator().next()
            val neighbours = graph.neighbours(v)
            if (clique.size() + 1 >= cliqueConsumer.minSize && Util.areDisjoint(neighbours, excluded)) {
                cliqueConsumer.accept(clique.plus(v))
            }
        } else if (pivotChoice == PivotChoice.Arbitrary) {
            val remainingCandidates = ArrayList<Vertex>(candidates)
            val pivot = remainingCandidates[0]
            visitAroundPivot(
                graph, cliqueConsumer, candidates, excluded, clique, PivotChoice.Arbitrary,
                pivot, remainingCandidates
            )
        } else {
            visitMaxDegree(graph, cliqueConsumer, candidates, excluded, clique, pivotChoice)
        }
    }

    private fun visitMaxDegree(
        graph: UndirectedGraph, cliqueConsumer: CliqueConsumer,
        candidates: MutableSet<Vertex>, excluded: MutableSet<Vertex>,
        clique: Clique, pivotChoice: PivotChoice
    ) {
        require(pivotChoice == PivotChoice.MaxDegreeLocal || pivotChoice == PivotChoice.MaxDegreeLocalX)
        // Quickly handle locally unconnected candidates while finding pivot
        var pivot = Vertex(0)
        val remainingCandidates: MutableCollection<Vertex> = ArrayList(candidates.size)
        var seenLocalDegree = 0
        for (v in candidates) {
            val neighbours = graph.neighbours(v)
            val localDegree = Util.overlap(neighbours, candidates)
            if (localDegree == 0) {
                // Same logic as below, stripped down
                if (clique.size() + 1 >= cliqueConsumer.minSize && Util.areDisjoint(neighbours, excluded)) {
                    cliqueConsumer.accept(clique.plus(v))
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
                val localDegree = Util.overlap(neighbours, candidates)
                if (seenLocalDegree < localDegree) {
                    seenLocalDegree = localDegree
                    pivot = v
                }
            }
        }
        visitAroundPivot(
            graph, cliqueConsumer, candidates, excluded, clique, pivotChoice, pivot, remainingCandidates
        )
    }

    private fun visitAroundPivot(
        graph: UndirectedGraph, cliqueConsumer: CliqueConsumer,
        candidates: MutableSet<Vertex>, excluded: MutableSet<Vertex>,
        clique: Clique, furtherPivotChoice: PivotChoice,
        pivot: Vertex, remainingCandidates: Iterable<Vertex>
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
                        clique = clique.plus(v)
                    )
                } else if (clique.size() + 1 >= cliqueConsumer.minSize && Util.areDisjoint(
                        neighbours,
                        excluded
                    )
                ) {
                    cliqueConsumer.accept(clique.plus(v))
                }
                excluded.add(v)
            }
        }
    }
}
