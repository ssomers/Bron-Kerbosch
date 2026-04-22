package be.steinsomers.bron_kerbosch

@JvmInline
value class Clique(val vertices: IntArray) {
    companion object Factory {
        fun empty(): Clique = Clique(intArrayOf())
        fun singleton(v: Vertex): Clique = Clique(intArrayOf(v.index))
    }

    fun plus(v: Vertex): Clique {
        return Clique(vertices + v.index)
    }

    fun size(): Int = vertices.size
}
