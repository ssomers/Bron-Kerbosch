package be.steinsomers.bron_kerbosch

@JvmInline
value class CliqueInProgress(val vertices: IntArray) {
    companion object Factory {
        fun empty(): CliqueInProgress = CliqueInProgress(intArrayOf())
        fun singleton(v: Vertex): CliqueInProgress = CliqueInProgress(intArrayOf(v.index))
    }

    fun plus(v: Vertex): CliqueInProgress {
        return CliqueInProgress(vertices + v.index)
    }

    fun size(): Int = vertices.size
}
