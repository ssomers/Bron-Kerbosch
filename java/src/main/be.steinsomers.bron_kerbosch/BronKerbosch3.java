package be.steinsomers.bron_kerbosch;

public final class BronKerbosch3 implements BronKerboschAlgorithm {
    @Override
    public void explore(final UndirectedGraph graph, final CliqueConsumer cliqueConsumer) {
        BronKerboschDegeneracy.explore(graph, cliqueConsumer, PivotChoice.Arbitrary);
    }
}
