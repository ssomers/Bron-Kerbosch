package be.steinsomers.bron_kerbosch

import java.util.concurrent.ArrayBlockingQueue
import java.util.concurrent.BlockingQueue

class BronKerbosch3MT : BronKerboschAlgorithm {
    companion object {
        private const val NUM_VISITING_THREADS = 5
    }

    @Throws(InterruptedException::class)
    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        val worker = Worker(graph, cliqueConsumer)
        worker.work()
    }

    private sealed class VisitJob {
        object CleanEnd : VisitJob()
        object DirtyEnd : VisitJob()
        data class Work(
            val startVertex: Vertex, val candidates: MutableSet<Vertex>, val excluded: MutableSet<Vertex>
        ) : VisitJob() {
            init {
                require(startVertex.index >= 0) // as if it would enable Kotlin to enumerate the end cases as negatives
            }
        }
    }

    private class Worker(private val graph: UndirectedGraph, private val cliqueConsumer: CliqueConsumer) {
        private val degeneracy = GraphDegeneracy(graph)
        private val visitQueue: BlockingQueue<VisitJob> = ArrayBlockingQueue(64)

        private inner class VisitProducer : Runnable {
            override fun run() {
                try {
                    degeneracy.forEach { v: Vertex ->
                        val (neighbouringCandidates, neighbouringExcluded) =
                            Util.partition(graph.neighbours(v)) { v -> degeneracy.isCandidate(v) }
                        Debug.assert { neighbouringCandidates.isNotEmpty() }
                        visitQueue.put(VisitJob.Work(v, neighbouringCandidates, neighbouringExcluded))
                    }
                    repeat(NUM_VISITING_THREADS) { _ -> visitQueue.put(VisitJob.CleanEnd) }
                } catch (_: InterruptedException) {
                    repeat(NUM_VISITING_THREADS) { _ -> visitQueue.put(VisitJob.DirtyEnd) }
                }
            }
        }

        private inner class Visitor : Runnable {
            override fun run() {
                while (true) {
                    when (val job = visitQueue.take()) {
                        is VisitJob.CleanEnd -> return
                        is VisitJob.DirtyEnd -> return
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

        @Throws(InterruptedException::class)
        fun work() {
            val visitorProducer = Thread(VisitProducer())
            val visitors = Array(NUM_VISITING_THREADS) { Thread(Visitor()) }
            visitorProducer.start()
            visitors.forEach { v -> v.start() }
            visitors.forEach { v -> v.join() }
        }
    }
}
