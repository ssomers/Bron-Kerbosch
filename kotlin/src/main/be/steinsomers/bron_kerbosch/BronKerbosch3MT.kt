package be.steinsomers.bron_kerbosch

import java.util.concurrent.ArrayBlockingQueue
import java.util.concurrent.BlockingQueue
import java.util.function.Consumer

class BronKerbosch3MT : BronKerboschAlgorithm {
    companion object {
        private const val NUM_VISITING_THREADS = 5
    }

    @Throws(InterruptedException::class)
    override fun explore(graph: UndirectedGraph, cliqueConsumer: (IntArray) -> Unit) {
        val worker = Worker(graph, cliqueConsumer)
        worker.work()
    }

    private sealed class StartJob {
        object CleanEnd : StartJob()
        object DirtyEnd : StartJob()
        data class Work(
            val startVertex: Int,
        ) : StartJob() {
            init {
                require(startVertex >= 0) // as if to enable Kotlin to enumerate the end cases as negatives
            }
        }
    }

    private sealed class VisitJob {
        object CleanEnd : VisitJob()
        object DirtyEnd : VisitJob()
        data class Work(
            val startVertex: Int, val candidates: MutableSet<Int>, val excluded: MutableSet<Int>
        ) : VisitJob() {
            init {
                require(startVertex >= 0) // as if to enable Kotlin to enumerate the end cases as negatives
            }
        }
    }

    private class Worker(private val graph: UndirectedGraph, private val cliqueConsumer: Consumer<IntArray>) {
        private val startQueue: BlockingQueue<StartJob> = ArrayBlockingQueue(64)
        private val visitQueue: BlockingQueue<VisitJob> = ArrayBlockingQueue(64)

        private inner class StartProducer : Runnable {
            override fun run() {
                try {
                    val vertices = Iterable { GraphDegeneracy(graph) }
                    for (v in vertices) {
                        startQueue.put(StartJob.Work(v))
                    }
                    startQueue.put(StartJob.CleanEnd)
                } catch (_: InterruptedException) {
                    startQueue.clear()
                    startQueue.add(StartJob.DirtyEnd)
                }
            }
        }

        private inner class VisitProducer : Runnable {
            override fun run() {
                try {
                    val excluded: MutableSet<Int> = HashSet(graph.order)
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
                                val v = job.startVertex
                                val neighbours = graph.neighbours(v)
                                require(neighbours.isNotEmpty())
                                val neighbouringCandidates = (neighbours subtract excluded).toMutableSet()
                                if (neighbouringCandidates.isEmpty()) {
                                    Debug.assert { !Util.areDisjoint(neighbours, excluded) }
                                } else {
                                    val neighbouringExcluded = Util.intersect(neighbours, excluded)
                                    visitQueue.put(
                                        VisitJob.Work(job.startVertex, neighbouringCandidates, neighbouringExcluded)
                                    )
                                }
                                excluded.add(v)
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
                                cliqueInProgress = intArrayOf(job.startVertex)
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
