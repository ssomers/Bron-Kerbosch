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
                .map { item -> visitProducer.createJob(item) }
                .toList() // TODO parallelize without intermediate list
                .parallelStream()
                .forEach(visitor::visit)
        }

        private inner class VisitProducer {
            fun createJob(item: GraphDegeneracyItem): VisitJob {
                val startVtx = item.pick
                val neighbouringExcluded = item.pickedNeighbours
                val neighbouringCandidates = graph.neighbours(startVtx).subtract(neighbouringExcluded).toMutableSet()
                return VisitJob.Work(startVtx, neighbouringCandidates, neighbouringExcluded)
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
