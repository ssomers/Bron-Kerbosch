package be.steinsomers.bron_kerbosch.study

import be.steinsomers.bron_kerbosch.UndirectedGraph
import java.io.IOException
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.util.stream.Stream

internal data class GraphTestData(val graph: UndirectedGraph, val cliqueCount: Int) {
    companion object {
        private fun newSets(n: Int): MutableList<MutableSet<Int>> {
            return Stream
                .generate { HashSet<Int>(16) as MutableSet<Int> }
                .limit(n.toLong())
                .toList()
        }

        @Throws(IOException::class)
        fun readUndirected(orderStr: String, order: Int, size: Int): GraphTestData {
            require(order > 2)
            require(size >= 0)
            val fullyMeshedSize = (order.toLong()) * (order - 1) / 2
            require(size <= fullyMeshedSize) { "$order nodes accommodate at most $fullyMeshedSize edges" }

            val edgesPath = Paths.get("..", "data", "random_edges_order_$orderStr.txt")
            val statsPath = Paths.get("..", "data", "random_stats.txt")
            val adjacencies = readEdges(edgesPath, order, size)
            val cliqueCount = readStats(statsPath, orderStr, size)

            val g = UndirectedGraph(adjacencies)
            require(g.order == order) { "order mishap" }
            require(g.size() == size) { "size mishap" }
            return GraphTestData(g, cliqueCount)
        }

        @Throws(IOException::class)
        private fun readEdges(path: Path, order: Int, size: Int): MutableList<MutableSet<Int>> {
            val adjacencies: MutableList<MutableSet<Int>> = newSets(order)
            Files.newBufferedReader(path).use { br ->
                for (lineNum in 0..<size) {
                    val line = br.readLine()
                    if (line == null)
                        throw IOException("File $path has only $lineNum lines of the $size requested")
                    val fields: Array<String> = line.split(" ".toRegex(), limit = 2).toTypedArray()
                    val v: Int
                    val w: Int
                    try {
                        v = fields[0].toInt()
                        w = fields[1].toInt()
                    } catch (_: NumberFormatException) {
                        throw IOException("File $path contains bogus text $line")
                    } catch (_: ArrayIndexOutOfBoundsException) {
                        throw IOException("File $path contains bogus text $line")
                    }
                    adjacencies[v].add(w)
                    adjacencies[w].add(v)
                }
            }
            return adjacencies
        }

        @Throws(IOException::class)
        private fun readStats(path: Path, orderStr: String, size: Int): Int {
            val prefix = "$orderStr\t$size\t"
            var cliqueCount = 0
            Files.newBufferedReader(path).use { br ->
                br.forEachLine { line ->
                    if (line.startsWith(prefix)) {
                        if (cliqueCount != 0)
                            throw IOException("File $path multiply defines order $orderStr size $size")
                        try {
                            cliqueCount = line.substring(prefix.length).toInt()
                        } catch (_: NumberFormatException) {
                            throw IOException("File $path has bogus line “$line”")
                        }
                        if (cliqueCount <= 0)
                            throw IOException("File $path has stupid line “$line”")
                    }
                }
            }
            if (cliqueCount == 0)
                throw IOException("File $path lacks order $orderStr size $size")
            return cliqueCount
        }
    }
}
