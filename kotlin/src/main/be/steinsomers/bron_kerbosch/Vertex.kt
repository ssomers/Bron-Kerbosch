package be.steinsomers.bron_kerbosch

@JvmInline
value class Vertex(val index: Int) : Comparable<Vertex> {
    override fun compareTo(other: Vertex): Int = index.compareTo(other.index)
    override fun toString(): String = "v$index"
}