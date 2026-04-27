package be.steinsomers.bron_kerbosch.study

import be.steinsomers.bron_kerbosch.UndirectedGraph
import be.steinsomers.bron_kerbosch.Vertex
import java.io.IOException
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths

internal data class KnownStats(
    private val cliqueCounts: List<Int>
) {
    fun cliqueCount(minSize: Int): Int {
        require(minSize >= 2)
        return cliqueCounts[minSize - 2]
    }
}

internal data class KnownRandomGraph(
    val graph: UndirectedGraph,
    val stats: KnownStats
) {
    companion object {
        @Throws(IOException::class)
        fun readUndirected(orderStr: String, order: Int, size: Int): KnownRandomGraph {
            require(order > 0)
            require(size >= 0)
            val fullyMeshedSize = (order.toLong()) * (order - 1) / 2
            require(size <= fullyMeshedSize) { "$order nodes accommodate at most $fullyMeshedSize edges" }

            val edgesPath = Paths.get("..", "data", "random_edges_order_$orderStr.txt")
            val statsPath = Paths.get("..", "data", "random_stats.txt")
            val adjacencies = readEdges(edgesPath, order, size)
            val stats = readStats(statsPath, orderStr, size)

            val g = UndirectedGraph(adjacencies)
            require(g.order == order)
            require(g.size == size)
            return KnownRandomGraph(g, stats)
        }

        @Throws(IOException::class)
        private fun readEdges(path: Path, order: Int, size: Int): MutableList<MutableSet<Vertex>> {
            val adjacencies: MutableList<MutableSet<Vertex>> = MutableList(order) { HashSet() }
            Files.newBufferedReader(path).use { br ->
                for (lineNum in 0..<size) {
                    val line = br.readLine()
                        ?: throw IOException("File $path has only $lineNum of the requested $size lines")
                    val fields: Array<String> = line.split(' ', limit = 2).toTypedArray()
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
                    adjacencies[v].add(Vertex(w))
                    adjacencies[w].add(Vertex(v))
                }
            }
            return adjacencies
        }

        @Throws(IOException::class)
        private fun readStats(path: Path, orderStr: String, size: Int): KnownStats {
            val prefix = "$orderStr\t$size\t"
            var cliqueCounts = emptyList<Int>()
            Files.newBufferedReader(path).use { br ->
                br.forEachLine { line ->
                    if (line.startsWith(prefix)) {
                        if (cliqueCounts.isNotEmpty())
                            throw IOException("File $path multiply defines order $orderStr size $size")
                        try {
                            cliqueCounts = line.substring(prefix.length).split('\t').map { c -> c.toInt() }
                        } catch (_: NumberFormatException) {
                            throw IOException("File $path has bogus line “$line”")
                        }
                        if (cliqueCounts.isEmpty() || cliqueCounts.any { c -> c < 0 })
                            throw IOException("File $path has stupid line “$line”")
                    }
                }
            }
            if (cliqueCounts.isEmpty())
                throw IOException("File $path lacks order $orderStr size $size")
            return KnownStats(cliqueCounts)
        }
    }
}
