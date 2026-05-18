package be.steinsomers.bron_kerbosch

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.channels.ReceiveChannel
import kotlinx.coroutines.channels.produce
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking

@Suppress("MemberVisibilityCanBePrivate")
class BronKerbosch3MT(val visitingThreads: Int) : BronKerboschAlgorithm {
    override val name: String = "Ver3½=GP$visitingThreads"
    override val deterministic: Boolean = false

    private fun log(msg: String) {
        // println(msg)
    }

    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        val dedicatedStorage = cliqueConsumer.storage.spawn(visitingThreads)
        var id = 0
        runBlocking {
            val channel = produceJobs(graph)
            dedicatedStorage.forEach {
                val ownCliqueConsumer = CliqueConsumer(minSize = cliqueConsumer.minSize, storage = it)
                launchVisitor(++id, graph, ownCliqueConsumer, channel)
            }
        }
        cliqueConsumer.storage.absorb(dedicatedStorage)
    }

    data class VisitJob(
        val startVertex: Vertex,
        val candidates: MutableSet<Vertex>,
        val excluded: MutableSet<Vertex>
    )

    private fun CoroutineScope.produceJobs(graph: UndirectedGraph): ReceiveChannel<VisitJob> =
        produce {
            val degeneracy = GraphDegeneracy(graph)
            degeneracy.forEach { v: Vertex ->
                val (neighbouringCandidates, neighbouringExcluded) =
                    Util.partition(graph.neighbours(v)) { v -> degeneracy.isCandidate(v) }
                Debug.assert { neighbouringCandidates.isNotEmpty() }
                send(VisitJob(v, neighbouringCandidates, neighbouringExcluded))
            }
        }

    private fun CoroutineScope.launchVisitor(
        id: Int,
        graph: UndirectedGraph,
        ownCliqueConsumer: CliqueConsumer,
        channel: ReceiveChannel<VisitJob>
    ) = launch(Dispatchers.Default) {
        log("visitor$id started")
        for (job in channel) {
            log("visitor$id started job ${job.startVertex}")
            async {
                BronKerboschPivot.visit(
                    graph = graph, cliqueConsumer = ownCliqueConsumer,
                    pivotChoice = PivotChoice.MaxDegreeLocal,
                    candidates = job.candidates,
                    excluded = job.excluded,
                    cliqueInProgress = Clique.singleton(job.startVertex)
                )
            }.await()
            log("visitor$id finished job ${job.startVertex}")
        }
        log("visitor$id ended")
    }
}
