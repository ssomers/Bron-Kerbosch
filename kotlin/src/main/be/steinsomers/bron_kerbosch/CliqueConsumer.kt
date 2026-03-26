package be.steinsomers.bron_kerbosch

data class CliqueConsumer(val minSize: Int, private val acceptor: (IntArray) -> Unit) {
    fun accept(clique: IntArray) {
        acceptor.invoke(clique)
    }
}
