package be.steinsomers.bron_kerbosch

class BronKerbosch3ST : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        val worker = Worker(graph, cliqueConsumer)
        worker.work()
    }

    private sealed class VisitJob {
        // Similar to BronKerbosch3MT but doesn't need sentinels
        data class Work(
            val startVertex: Vertex,
            val candidates: MutableSet<Vertex>,
            val excluded: MutableSet<Vertex>
        ) : VisitJob()
    }

    private class Worker(private val graph: UndirectedGraph, private val cliqueConsumer: CliqueConsumer) {
        fun work() {
            val visitProducer = VisitProducer()
            val visitor = Visitor()
            val ordering = GraphDegeneracy(graph)
            return ordering.asSequence()
                .map { item -> visitProducer.createJobIfNeeded(item) }
                .toList() // TODO parallelize without intermediate list
                .parallelStream()
                .forEach { job: VisitJob? -> if (job != null) visitor.visit(job) }
        }

        private inner class VisitProducer {
            fun createJobIfNeeded(item: GraphDegeneracyItem): VisitJob? {
                var job: VisitJob? = null
                val startVtx = item.pick
                val neighbouringExcluded = item.pickedNeighbours
                val neighbours = graph.neighbours(startVtx)
                Debug.assert { neighbours.isNotEmpty() }
                if (neighbouringExcluded.size < neighbours.size) {
                    val neighbouringCandidates = neighbours.subtract(neighbouringExcluded).toMutableSet()
                    job = VisitJob.Work(startVtx, neighbouringCandidates, neighbouringExcluded)
                }
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
                            cliqueInProgress = CliqueInProgress.singleton(job.startVertex)
                        )
                }
            }
        }
    }
}
