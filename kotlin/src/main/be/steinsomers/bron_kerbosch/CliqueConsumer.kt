package be.steinsomers.bron_kerbosch

data class CliqueConsumer(val minSize: Int, private val acceptor: (CliqueInProgress) -> Unit) {
    fun accept(clique: CliqueInProgress) {
        acceptor.invoke(clique)
    }
}
