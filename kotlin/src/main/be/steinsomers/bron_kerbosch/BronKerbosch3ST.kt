package be.steinsomers.bron_kerbosch

class BronKerbosch3ST : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        val worker = Worker(graph, cliqueConsumer)
        worker.work()
    }

    private sealed class VisitJob {
        // Similar to BronKerbosch3MT but doesn't need sentinels
        data class Work(
            val startVertex: Int,
            val candidates: MutableSet<Int>,
            val excluded: MutableSet<Int>
        ) : VisitJob()
    }

    private class Worker(private val graph: UndirectedGraph, private val cliqueConsumer: CliqueConsumer) {
        fun work() {
            val visitProducer = VisitProducer()
            val visitor = Visitor()
            val ordering = GraphDegeneracy(graph)
            return ordering.asSequence()
                .map { startVtx: Int -> visitProducer.createJobIfNeeded(startVtx) }
                .toList() // TODO parallelize without intermediate list
                .parallelStream()
                .forEach { job: VisitJob? -> if (job != null) visitor.visit(job) }
        }

        private inner class VisitProducer {
            private val excluded = BooleanArray(graph.order)

            fun createJobIfNeeded(startVtx: Int): VisitJob? {
                var job: VisitJob? = null
                val neighbours = graph.neighbours(startVtx)
                Debug.assert { neighbours.isNotEmpty() }
                val neighbouringCandidates = neighbours.filterNotTo(HashSet()) { v -> excluded[v] }
                if (!neighbouringCandidates.isEmpty()) {
                    val neighbouringExcluded = neighbours.filterTo(HashSet()) { v -> excluded[v] }
                    job = VisitJob.Work(startVtx, neighbouringCandidates, neighbouringExcluded)
                }
                excluded[startVtx] = true
                return job
            }
        }

        private inner class Visitor {
            fun visit(job: VisitJob) {
                when (job) {
                    is VisitJob.Work ->
                        BronKerboschPivot.visit(
                            graph = graph, cliqueConsumer = cliqueConsumer,
                            pivotChoice = PivotChoice.MaxDegreeLocal,
                            candidates = job.candidates,
                            excluded = job.excluded,
                            cliqueInProgress = intArrayOf(job.startVertex)
                        )
                }
            }
        }
    }
}
