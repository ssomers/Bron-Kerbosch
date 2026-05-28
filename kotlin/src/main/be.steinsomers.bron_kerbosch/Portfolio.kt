package be.steinsomers.bron_kerbosch

internal object Portfolio {
    val ALGOS: List<BronKerboschAlgorithm> = listOf(
        BronKerbosch1(),
        BronKerbosch2(), BronKerbosch2gp(), BronKerbosch2gpx(),
        BronKerbosch3(), BronKerbosch3gp(), BronKerbosch3gpx(),
        BronKerbosch3MT(1), BronKerbosch3MT(4), BronKerbosch3MT(5), BronKerbosch3MT(6), BronKerbosch3MT(8),
        BronKerbosch3MT(16), BronKerbosch3MT(64), BronKerbosch3MT(256),
        BronKerbosch3ST()
    )
}
