package be.steinsomers.bron_kerbosch

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.channels.ReceiveChannel
import kotlinx.coroutines.channels.produce
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlin.coroutines.CoroutineContext

@Suppress("MemberVisibilityCanBePrivate")
class BronKerbosch3MT(val visitingCoroutines: Int) : BronKerboschAlgorithm {
    override val name: String = "Ver3½=GP$visitingCoroutines"
    override val deterministic: Boolean = false

    private fun log(msg: String) {
        // println(msg)
    }

    override fun explore(graph: UndirectedGraph, cliqueConsumer: CliqueConsumer) {
        val context: CoroutineContext = Dispatchers.Default
        val dedicatedStorage = cliqueConsumer.storage.spawn(visitingCoroutines)
        var id = 0
        runBlocking {
            val channel = produceJobs(context, graph)
            dedicatedStorage.forEach {
                val ownCliqueConsumer = CliqueConsumer(minSize = cliqueConsumer.minSize, storage = it)
                launchVisitor(context, ++id, graph, ownCliqueConsumer, channel)
            }
        }
        cliqueConsumer.storage.absorb(dedicatedStorage)
    }

    data class VisitJob(
        val startVertex: Vertex,
        val candidates: MutableSet<Vertex>,
        val excluded: MutableSet<Vertex>
    )

    private fun CoroutineScope.produceJobs(
        context: CoroutineContext,
        graph: UndirectedGraph
    ): ReceiveChannel<VisitJob> =
        produce(context = context) {
            val degeneracy = GraphDegeneracy(graph)
            degeneracy.forEach { v: Vertex ->
                val (neighbouringCandidates, neighbouringExcluded) =
                    Util.partition(graph.neighbours(v)) { v -> degeneracy.isCandidate(v) }
                Debug.assert { neighbouringCandidates.isNotEmpty() }
                send(VisitJob(v, neighbouringCandidates, neighbouringExcluded))
            }
        }

    private fun CoroutineScope.launchVisitor(
        context: CoroutineContext,
        id: Int,
        graph: UndirectedGraph,
        ownCliqueConsumer: CliqueConsumer,
        channel: ReceiveChannel<VisitJob>
    ) = launch(context = context) {
        log("visitor$id started")
        for (job in channel) {
            log("visitor$id started job ${job.startVertex}")
            BronKerboschPivot.visit(
                graph = graph, cliqueConsumer = ownCliqueConsumer,
                pivotChoice = PivotChoice.MaxDegreeLocal,
                candidates = job.candidates,
                excluded = job.excluded,
                cliqueInProgress = Clique.singleton(job.startVertex)
            )
            log("visitor$id finished job ${job.startVertex}")
        }
        log("visitor$id ended")
    }
}
