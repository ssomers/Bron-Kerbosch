package be.steinsomers.bron_kerbosch.study

import be.steinsomers.bron_kerbosch.*
import java.io.IOException
import java.nio.charset.StandardCharsets
import java.nio.file.Files
import java.nio.file.Paths
import java.util.*
import java.util.stream.IntStream

internal object Main {
    private const val CLIQUE_MIN_SIZE = 3

    data class TimedAlgo(val algo: BronKerboschAlgorithm, var time: SampleStatistics)

    @Throws(InterruptedException::class)
    private fun bkTimed(testData: KnownRandomGraph, timedSamples: Int, measurements: List<TimedAlgo>) {
        val expectedCount = testData.stats.cliqueCount(minSize = CLIQUE_MIN_SIZE)
        var firstOrdered: Optional<List<SortedClique>> = Optional.empty()
        for (sample in 0..timedSamples) {
            for (measurement in measurements) {
                if (sample == 0) {
                    val cliqueCollector = CliqueCollector()
                    measurement.algo.explore(testData.graph, CliqueConsumer(minSize = CLIQUE_MIN_SIZE, cliqueCollector))
                    val ordered = cliqueCollector.toSortedList()
                    if (firstOrdered.isEmpty) {
                        require(ordered.size == expectedCount) {
                            "Initial sample obtained ${ordered.size} cliques, expected $expectedCount"
                        }
                        firstOrdered = Optional.of(ordered)
                    } else {
                        require(firstOrdered.get() == ordered) { "Inconsistent results" }
                    }
                } else {
                    val start = System.nanoTime()
                    val cliqueCounter = CliqueCounter()
                    val cliqueConsumer = CliqueConsumer(3, cliqueCounter)
                    measurement.algo.explore(testData.graph, cliqueConsumer)
                    val elapsed = System.nanoTime() - start
                    val obtainedCount = cliqueCounter.harvest()
                    require(obtainedCount == expectedCount) {
                        "Sample $sample obtained $obtainedCount cliques, expected $expectedCount"
                    }
                    measurement.time.put(elapsed)
                }
            }
        }
    }

    private fun bk(
        genuine: Boolean,
        orderStr: String,
        order: Int,
        sizes: IntArray,
        samples: Int,
        algos: List<BronKerboschAlgorithm>
    ) {
        val name = "random_time_kotlin_order_" + (if (genuine) orderStr else "warmup")
        val path = Paths.get("..", "data", "$name.csv")
        try {
            Files.newBufferedWriter(path, StandardCharsets.UTF_8).use { fo ->
                fo.write("Size")
                algos.forEach {
                    fo.write(String.format(Locale.US, ",%s min,%s mean,%s max", it.name, it.name, it.name))
                }
                fo.write(System.lineSeparator())
                for (size in sizes) {
                    val start = System.nanoTime()
                    val testData = KnownRandomGraph.readUndirected(orderStr, order, size)
                    val elapsed = System.nanoTime() - start
                    if (genuine) {
                        val knownCliqueCount = testData.stats.cliqueCount(minSize = CLIQUE_MIN_SIZE)
                        System.out.printf(
                            "%4s nodes, %7d edges, %5d cliques, %d samples, creation: %6.3f%n",
                            orderStr, size, knownCliqueCount, samples, elapsed / 1e9
                        )
                    }
                    val measurements = algos.map { algo -> TimedAlgo(algo = algo, time = SampleStatistics()) }
                    bkTimed(testData, samples, measurements)

                    fo.write(String.format(Locale.US, "%d", size))
                    for (mm in measurements) {
                        val max = mm.time.max() / 1e9
                        val min = mm.time.min() / 1e9
                        val mean = mm.time.mean() / 1e9
                        val dev = mm.time.deviation() / 1e9
                        fo.write(String.format(Locale.US, ",%f,%f,%f", min, mean, max))
                        if (genuine) {
                            System.out.printf(
                                "%4s nodes, %7d edges, %8s: %6.3f ± %.0f%%%n",
                                orderStr, size, mm.algo.name, mean, 100 * dev / mean
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

        val allAlgos = Portfolio.ALGOS
        val mostAlgos = allAlgos.filterNot { it is BronKerbosch1 }
        val eliteAlgos = allAlgos.filter {
            it is BronKerbosch2gp
                    || it is BronKerbosch3gp
                    || it is BronKerbosch3MT && it.visitingThreads <= 6
                    || it is BronKerbosch3ST
        }
        val sizes100 = IntStream.iterate(2_000, { s -> s <= 3_000 }, { s -> s + 50 }).toArray()
        val sizes10K = IntStream.iterate(
            10_000, { s -> s <= 200_000 },
            { s -> s + (if (s < 100_000) 10_000 else 25_000) }).toArray()
        val sizes1M = IntStream.iterate(
            500_000, { s -> s <= 5_000_000 },
            { s -> s + (if (s < 2_000_000) 250_000 else 1_000_000) }).toArray()

        // First warm up.
        bk(genuine = false, orderStr = "100", order = 100, sizes = intArrayOf(2_000), samples = 3, algos = allAlgos)
        Thread.sleep(3210) // give IntelliJ launcher some time to cool down
        bk(genuine = true, orderStr = "100", order = 100, sizes = sizes100, samples = 5, algos = allAlgos)
        bk(genuine = true, orderStr = "10k", order = 10_000, sizes = sizes10K, samples = 3, algos = mostAlgos)
        bk(genuine = true, orderStr = "1M", order = 1_000_000, sizes = sizes1M, samples = 3, algos = eliteAlgos)
    }
}
