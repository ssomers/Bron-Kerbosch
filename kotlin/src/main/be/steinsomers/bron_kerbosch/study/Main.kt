package be.steinsomers.bron_kerbosch.study

import be.steinsomers.bron_kerbosch.*
import java.io.IOException
import java.nio.charset.StandardCharsets
import java.nio.file.Files
import java.nio.file.Paths
import java.util.*
import java.util.stream.IntStream
import kotlin.collections.ArrayDeque
import kotlin.concurrent.atomics.AtomicInt
import kotlin.concurrent.atomics.ExperimentalAtomicApi
import kotlin.math.ceil
import kotlin.math.min
import kotlin.math.sqrt

internal object Main {
    val FUNC_NAMES: Array<String> = arrayOf(
        "Ver1½",
        "Ver2½",
        "Ver2½-GP",
        "Ver2½-GPX",
        "Ver3½",
        "Ver3½-GP",
        "Ver3½-GPX",
        "Ver3½=GPc",
        "Ver3½=GPs",
    )
    val FUNCS: Array<BronKerboschAlgorithm> = arrayOf(
        BronKerbosch1(),
        BronKerbosch2(),
        BronKerbosch2gp(),
        BronKerbosch2gpx(),
        BronKerbosch3(),
        BronKerbosch3gp(),
        BronKerbosch3gpx(),
        BronKerbosch3MT(),
        BronKerbosch3ST(),
    )

    fun orderCliques(cliques: Collection<IntArray>): List<List<Int>> {
        require(cliques.all { clique -> clique.size > 1 })
        return cliques
            .map(IntArray::sorted)
            .sortedWith { clique1: List<Int>, clique2: List<Int> ->
                when (val diff = (0..<min(clique1.size, clique2.size)).asSequence()
                    .map { i -> clique1[i] - clique2[i] }
                    .firstOrNull { diff -> diff != 0 }) {
                    null -> throw IllegalArgumentException("got overlapping or equal cliques $clique1 <> $clique2")
                    else -> diff
                }
            }
            .toList()
    }

    @OptIn(ExperimentalAtomicApi::class)
    @Throws(InterruptedException::class)
    private fun bkTimed(
        testData: GraphTestData,
        timedSamples: Int, funcIndices: IntArray
    ): Array<SampleStatistics> {
        var firstOrdered: Optional<List<List<Int>>> = Optional.empty()
        val times = Array(FUNCS.size) { _ -> SampleStatistics() }
        IntStream.range(0, FUNCS.size).forEach { i: Int -> times[i] = SampleStatistics() }
        for (sample in 0..timedSamples) {
            for (funcIndex in funcIndices) {
                if (sample == 0) {
                    val initialCap = ceil(sqrt(testData.graph.size.toDouble())).toInt()
                    val cliques = Collections.synchronizedCollection(ArrayDeque<IntArray>(initialCap))
                    FUNCS[funcIndex].explore(testData.graph, cliques::add)
                    val ordered = orderCliques(cliques)
                    if (firstOrdered.isEmpty) {
                        require(
                            cliques.size == testData.cliqueCount
                        ) { "Got ${cliques.size} cliques, expected ${testData.cliqueCount}" }
                        firstOrdered = Optional.of(ordered)
                    } else {
                        require(firstOrdered.get() == ordered) { "Inconsistent results" }
                    }
                } else {
                    val start = System.nanoTime()
                    val cliqueCounter = AtomicInt(0)
                    FUNCS[funcIndex].explore(testData.graph) { _ -> cliqueCounter.addAndFetch(1) }
                    val elapsed = System.nanoTime() - start
                    val cliqueCount = cliqueCounter.load()
                    require(
                        cliqueCount == testData.cliqueCount
                    ) { "Got $cliqueCount cliques after sample $sample, expected ${testData.cliqueCount}" }
                    times[funcIndex].put(elapsed)
                }
            }
        }
        return times
    }

    private fun bk(
        genuine: Boolean,
        orderStr: String,
        order: Int,
        sizes: IntArray,
        samples: Int,
        funcIndices: IntArray
    ) {
        val name = "bron_kerbosch_kotlin_order_" + (if (genuine) orderStr else "warmup")
        val path = Paths.get("..", "$name.csv")
        try {
            Files.newBufferedWriter(path, StandardCharsets.UTF_8).use { fo ->
                fo.write("Size")
                for (funcIndex in funcIndices) {
                    val fn = FUNC_NAMES[funcIndex]
                    fo.write(String.format(Locale.US, ",%s min,%s mean,%s max", fn, fn, fn))
                }
                fo.write(System.lineSeparator())
                for (size in sizes) {
                    val start = System.nanoTime()
                    val testData: GraphTestData = GraphTestData.readUndirected(orderStr, order, size)
                    val elapsed = System.nanoTime() - start
                    if (genuine) {
                        System.out.printf(
                            "%4s nodes, %7d edges, creation: %6.3f%n",
                            orderStr, size, elapsed / 1e9
                        )
                    }
                    val times = bkTimed(testData, samples, funcIndices)

                    fo.write(String.format(Locale.US, "%d", size))
                    for (funcIndex in funcIndices) {
                        val funcName = FUNC_NAMES[funcIndex]
                        val max = times[funcIndex].max() / 1e9
                        val min = times[funcIndex].min() / 1e9
                        val mean = times[funcIndex].mean() / 1e9
                        val dev = times[funcIndex].deviation() / 1e9
                        fo.write(String.format(Locale.US, ",%f,%f,%f", min, mean, max))
                        if (genuine) {
                            System.out.printf(
                                "%4s nodes, %7d edges, %8s: %6.3f ± %.0f%%%n",
                                orderStr, size, funcName, mean, 100 * dev / mean
                            )
                        }
                    }
                    fo.write(System.lineSeparator())
                }
            }
        } catch (x: InterruptedException) {
            System.err.format("InterruptedException: %s%n", x)
        } catch (x: IOException) {
            System.err.format("IOException: %s%n", x)
        }
    }

    @JvmStatic
    fun main(@Suppress("unused", "RedundantSuppression") args: Array<String>) {
        Debug.assert({ false }, { "Omit -ea for meaningful measurements" })

        val allFuncIndices = FUNCS.indices.toList().toIntArray()
        val mostFuncIndices = (1..<FUNCS.size).toList().toIntArray()
        val sizes100 = IntStream.iterate(2_000, { s -> s <= 3_000 }, { s -> s + 50 }).toArray()
        val sizes10K = IntStream.iterate(
            10_000, { s -> s <= 200_000 },
            { s -> s + (if (s < 100_000) 10_000 else 25_000) }).toArray()
        val sizes1M = IntStream.iterate(
            500_000, { s -> s <= 5_000_000 },
            { s -> s + (if (s < 2_000_000) 250_000 else 1_000_000) }).toArray()

        // First warm up.
        bk(false, "100", order = 100, sizes = intArrayOf(2_000), samples = 3, funcIndices = allFuncIndices)
        Thread.sleep(3210) // give IntelliJ launcher some time to cool down
        /*
                bk(true, "10k", order = 10_000, sizes = intArrayOf(200_000), samples = 5, funcIndices = intArrayOf(7))
                return
         */
        bk(true, "100", order = 100, sizes = sizes100, samples = 5, funcIndices = allFuncIndices)
        bk(true, "10k", order = 10_000, sizes = sizes10K, samples = 3, funcIndices = mostFuncIndices)
        bk(true, "1M", order = 1_000_000, sizes = sizes1M, samples = 3, funcIndices = intArrayOf(2, 5, 7, 8))
    }
}
