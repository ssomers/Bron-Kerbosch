package be.steinsomers.bron_kerbosch

internal object Portfolio {
    val ALGOS: List<BronKerboschAlgorithm> = listOf(
        BronKerbosch1(),
        BronKerbosch2(), BronKerbosch2gp(), BronKerbosch2gpx(),
        BronKerbosch3(), BronKerbosch3gp(), BronKerbosch3gpx(),
        BronKerbosch3MT(), BronKerbosch3ST()
    )
}
