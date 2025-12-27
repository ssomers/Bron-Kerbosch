package be.steinsomers.bron_kerbosch

import java.util.Collections
import java.util.concurrent.ArrayBlockingQueue
import java.util.concurrent.BlockingQueue
import java.util.stream.Stream
import kotlin.math.floor
import kotlin.math.sqrt

class BronKerbosch3MT : BronKerboschAlgorithm {
    companion object {
        private const val NUM_VISITING_THREADS = 5
    }

    @Throws(InterruptedException::class)
    override fun explore(graph: UndirectedGraph): Stream<IntArray> {
        val worker = Worker(graph)
        return worker.stream()
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
            val startVertex: Int,
            val candidates: MutableSet<Int>,
            val excluded: MutableSet<Int>
        ) : VisitJob() {
            init {
                require(startVertex >= 0) // as if to enable Kotlin to enumerate the end cases as negatives
            }
        }
    }

    private class Worker(private val graph: UndirectedGraph) {
        private val startQueue: BlockingQueue<StartJob>
        private val visitQueue: BlockingQueue<VisitJob>
        private val cliques: MutableCollection<IntArray>

        init {
            val initialCap = floor(sqrt(graph.size().toDouble())).toInt()
            cliques = Collections.synchronizedCollection(ArrayDeque(initialCap))
            startQueue = ArrayBlockingQueue(64)
            visitQueue = ArrayBlockingQueue(64)
        }

        private inner class StartProducer : Runnable {
            override fun run() {
                try {
                    val vertices = Iterable { DegeneracyOrdering(graph, drop=1) }
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
                                val neighbouringCandidates = neighbours subtract excluded
                                if (neighbouringCandidates.isEmpty()) {
                                    Debug.assert { !Util.areDisjoint(neighbours, excluded) }
                                } else {
                                    val neighbouringExcluded = Util.intersect(neighbours, excluded)
                                    visitQueue.put(
                                        VisitJob.Work(
                                            job.startVertex,
                                            neighbouringCandidates.toMutableSet(),
                                            neighbouringExcluded.toMutableSet()
                                        )
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
                try {
                    var job: VisitJob
                    while (true) {
                        job = visitQueue.take()
                        when (job) {
                            is VisitJob.CleanEnd -> return
                            is VisitJob.DirtyEnd -> return
                            is VisitJob.Work -> {
                                BronKerboschPivot.visit(
                                    graph, { e: IntArray -> cliques.add(e) },
                                    pivotChoice=PivotChoice.MaxDegreeLocal,
                                    candidates=job.candidates,
                                    excluded=job.excluded,
                                    cliqueInProgress=intArrayOf(job.startVertex)
                                )
                            }
                        }
                    }
                } catch (_: InterruptedException) {
                    cliques.clear()
                }
            }
        }

        @Throws(InterruptedException::class)
        fun stream(): Stream<IntArray> {
            val startProducer = Thread(StartProducer())
            val visitorProducer = Thread(VisitProducer())
            val visitors = Array(NUM_VISITING_THREADS) { Thread(Visitor()) }
            startProducer.start()
            visitorProducer.start()
            visitors.forEach { v -> v.start() }
            visitors.forEach { v -> v.join() }
            return cliques.stream()
        }
    }
}
