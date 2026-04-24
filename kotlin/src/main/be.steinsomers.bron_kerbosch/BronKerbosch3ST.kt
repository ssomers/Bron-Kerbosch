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
        private val degeneracy = GraphDegeneracy(graph)

        fun work() {
            val visitProducer = VisitProducer()
            val visitor = Visitor()
            val storage = degeneracy.asSequence()
                .map { pick -> visitProducer.createJob(pick) }
                .toList() // TODO parallelize without intermediate list
                .parallelStream()
                .map(visitor::visit)
                .toList()
            cliqueConsumer.storage.absorb(storage)
        }

        private inner class VisitProducer {
            fun createJob(startVtx: Vertex): VisitJob {
                val (neighbouringCandidates, neighbouringExcluded) =
                    Util.partition(graph.neighbours(startVtx)) { v -> degeneracy.isCandidate(v) }
                Debug.assert { neighbouringCandidates.isNotEmpty() }
                return VisitJob.Work(startVtx, neighbouringCandidates, neighbouringExcluded)
            }
        }

        private inner class Visitor {
            fun visit(job: VisitJob): CliqueStorage {
                when (job) {
                    is VisitJob.Work -> {
                        val storage = cliqueConsumer.storage.spawn(1)[0]
                        BronKerboschPivot.visit(
                            graph = graph,
                            cliqueConsumer = CliqueConsumer(minSize = cliqueConsumer.minSize, storage = storage),
                            pivotChoice = PivotChoice.MaxDegreeLocal,
                            candidates = job.candidates,
                            excluded = job.excluded,
                            cliqueInProgress = Clique.singleton(job.startVertex)
                        )
                        return storage
                    }
                }
            }
        }
    }
}
