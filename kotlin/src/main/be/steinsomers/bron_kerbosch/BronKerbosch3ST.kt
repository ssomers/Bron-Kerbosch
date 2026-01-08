package be.steinsomers.bron_kerbosch

import java.util.*
import java.util.function.IntFunction

class BronKerbosch3ST : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph, cliqueConsumer: (IntArray) -> Unit) {
        val worker = Worker(graph, cliqueConsumer)
        worker.work()
    }

    private sealed class VisitJob {
        // Similar to BronKerbosch3_MT but doesn't need sentinels
        data class Work(
            val startVertex: Int,
            val candidates: MutableSet<Int>,
            val excluded: MutableSet<Int>
        ) : VisitJob()
    }

    private class Worker(private val graph: UndirectedGraph, private val cliqueConsumer: (IntArray) -> Unit) {
        fun work() {
            val visitProducer = VisitProducer()
            val visitor = Visitor()
            val ordering = DegeneracyOrdering(graph, drop = 1)
            return ordering.stream()
                .mapToObj<VisitJob?>(IntFunction { startVtx: Int -> visitProducer.createJob(startVtx) })
                .filter { job: VisitJob? -> Objects.nonNull(job) }
                .toList()
                .parallelStream()
                .forEach { job: VisitJob -> visitor.visit(job) }
        }

        private inner class VisitProducer {
            private val excluded: MutableSet<Int> = HashSet(graph.order)

            fun createJob(startVtx: Int): VisitJob? {
                var job: VisitJob? = null
                val neighbours = graph.neighbours(startVtx)
                require(neighbours.isNotEmpty())
                val neighbouringCandidates = (neighbours subtract excluded).toMutableSet()
                if (neighbouringCandidates.isEmpty()) {
                    Debug.assert { !Util.areDisjoint(neighbours, excluded) }
                } else {
                    val neighbouringExcluded = Util.intersect(neighbours, excluded)
                    job = VisitJob.Work(startVtx, neighbouringCandidates, neighbouringExcluded)
                }
                excluded.add(startVtx)
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
