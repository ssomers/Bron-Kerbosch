package be.steinsomers.bron_kerbosch

import java.util.Objects
import java.util.function.IntFunction
import java.util.stream.Stream

class BronKerbosch3ST : BronKerboschAlgorithm {
    override fun explore(graph: UndirectedGraph): Stream<IntArray> {
        val worker = Worker(graph)
        return worker.stream()
    }

    private sealed class VisitJob {
        // Similar to BronKerbosch3_MT but doesn't need sentinels
        data class Work(
            val startVertex: Int,
            val candidates: MutableSet<Int>,
            val excluded: MutableSet<Int>
        ) : VisitJob()
    }

    private class Worker(private val graph: UndirectedGraph) {
        fun stream(): Stream<IntArray> {
            val visitProducer = VisitProducer()
            val visitor = Visitor()
            val ordering = DegeneracyOrdering(graph, drop = 1)
            return ordering.stream()
                .mapToObj<VisitJob?>(IntFunction { startVtx: Int -> visitProducer.createJob(startVtx) })
                .filter { job: VisitJob? -> Objects.nonNull(job) }
                .toList()
                .parallelStream()
                .flatMap { job: VisitJob? -> visitor.visit(job!!) }
        }

        private inner class VisitProducer {
            private val excluded: MutableSet<Int> = HashSet(graph.order)

            fun createJob(startVtx: Int): VisitJob? {
                var job: VisitJob? = null
                val neighbours = graph.neighbours(startVtx)
                require(neighbours.isNotEmpty())
                val neighbouringCandidates = neighbours subtract excluded
                if (neighbouringCandidates.isEmpty()) {
                    Debug.assert { !Util.areDisjoint(neighbours, excluded) }
                } else {
                    val neighbouringExcluded = Util.intersect(neighbours, excluded)
                    job = VisitJob.Work(
                        startVtx,
                        neighbouringCandidates.toMutableSet(),
                        neighbouringExcluded.toMutableSet()
                    )
                }
                excluded.add(startVtx)
                return job
            }
        }

        private inner class Visitor {
            fun visit(job: VisitJob): Stream<IntArray> {
                val cliqueStream = Stream.builder<IntArray>()
                when (job) {
                    is VisitJob.Work ->
                        BronKerboschPivot.visit(
                            graph, cliqueStream,
                            pivotChoice = PivotChoice.MaxDegreeLocal,
                            candidates = job.candidates,
                            excluded = job.excluded,
                            cliqueInProgress = intArrayOf(job.startVertex)
                        )
                }
                return cliqueStream.build()
            }
        }
    }
}
