package be.steinsomers.bron_kerbosch

// Could be value class, but Kotlin does not define structural equality on it and does not allow defining it.
data class SortedClique(val vertices: IntArray) {
    companion object Factory {
        fun freeze(clique: Clique): SortedClique = SortedClique(clique.vertices.sorted().toIntArray())
    }

    fun size(): Int = vertices.size

    override fun equals(other: Any?): Boolean = other is SortedClique && vertices contentEquals other.vertices
    override fun hashCode(): Int = vertices.contentHashCode()
}