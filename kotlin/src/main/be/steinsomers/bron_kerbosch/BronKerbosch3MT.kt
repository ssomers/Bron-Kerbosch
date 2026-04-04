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

    private sealed class StartJob {
        object CleanEnd : StartJob()
        object DirtyEnd : StartJob()
        data class Work(
            val startVertex: Vertex,
        ) : StartJob() {
            init {
                require(startVertex.index >= 0) // as if it would enable Kotlin to enumerate the end cases as negatives
            }
        }
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
        private val startQueue: BlockingQueue<StartJob> = ArrayBlockingQueue(64)
        private val visitQueue: BlockingQueue<VisitJob> = ArrayBlockingQueue(64)

        private inner class StartProducer : Runnable {
            override fun run() {
                try {
                    val vertices = Iterable { GraphDegeneracy(graph) }
                    for (v in vertices) {
                        startQueue.put(StartJob.Work(Vertex(v)))
                    }
                    startQueue.put(StartJob.CleanEnd)
                } catch (_: InterruptedException) {
                    startQueue.clear()
                    startQueue.add(StartJob.DirtyEnd)
                }
            }
        }

        private inner class VisitProducer : Runnable {
            private val excluded = BooleanArray(graph.order)

            private fun process(startVtx: Vertex) {
                val neighbours = graph.neighbours(startVtx)
                Debug.assert { neighbours.isNotEmpty() }
                val neighbouringCandidates =
                    neighbours.filterNotTo(HashSet()) { v -> excluded[v.index] }
                if (!neighbouringCandidates.isEmpty()) {
                    val neighbouringExcluded = neighbours.filterTo(HashSet()) { v -> excluded[v.index] }
                    visitQueue.put(VisitJob.Work(startVtx, neighbouringCandidates, neighbouringExcluded))
                }
                excluded[startVtx.index] = true
            }

            override fun run() {
                try {
                    while (true) {
                        when (val job = startQueue.take()) {
                            is StartJob.CleanEnd -> {
                                repeat(NUM_VISITING_THREADS) { _ -> visitQueue.put(VisitJob.CleanEnd) }
                                return
                            }

                            is StartJob.DirtyEnd -> {
                                repeat(NUM_VISITING_THREADS) { _ -> visitQueue.put(VisitJob.DirtyEnd) }
                                return
                            }

                            is StartJob.Work -> {
                                process(job.startVertex)
                            }
                        }
                    }
                } catch (_: InterruptedException) {
                    visitQueue.clear()
                    repeat(NUM_VISITING_THREADS) { _ -> visitQueue.put(VisitJob.DirtyEnd) }
                }
            }
        }

        private inner class Visitor : Runnable {
            override fun run() {
                var job: VisitJob
                while (true) {
                    job = visitQueue.take()
                    when (job) {
                        is VisitJob.CleanEnd -> return
                        is VisitJob.DirtyEnd -> return
                        is VisitJob.Work -> {
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

        @Throws(InterruptedException::class)
        fun work() {
            val startProducer = Thread(StartProducer())
            val visitorProducer = Thread(VisitProducer())
            val visitors = Array(NUM_VISITING_THREADS) { Thread(Visitor()) }
            startProducer.start()
            visitorProducer.start()
            visitors.forEach { v -> v.start() }
            visitors.forEach { v -> v.join() }
        }
    }
}
