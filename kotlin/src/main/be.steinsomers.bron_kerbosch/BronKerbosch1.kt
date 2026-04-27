// Naive Bron-Kerbosch algorithm
package be.steinsomers.bron_kerbosch

class BronKerbosch1 : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        val candidates: MutableSet<Vertex> = graph.connectedVertices().toCollection(HashSet())
        val excluded: MutableSet<Vertex> = HashSet(candidates.size)
        visit(
            graph = graph, cliqueConsumer = cliqueConsumer,
            candidates = candidates, excluded = excluded,
            clique = Clique.empty()
        )
    }

    companion object {
        private fun visit(
            graph: UndirectedGraph, cliqueConsumer: CliqueConsumer,
            candidates: MutableSet<Vertex>, excluded: MutableSet<Vertex>,
            clique: Clique
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
                        clique = clique.plus(v)
                    )
                } else if (clique.size() + 1 >= cliqueConsumer.minSize && Util.areDisjoint(
                        excluded,
                        neighbours
                    )
                ) {
                    cliqueConsumer.accept(clique.plus(v))
                }
                excluded.add(v)
            }
        }
    }
}
