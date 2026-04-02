package be.steinsomers.bron_kerbosch;

public final class BronKerbosch2_gp implements BronKerboschAlgorithm {
    @Override
    public void explore(final UndirectedGraph graph, final CliqueConsumer cliqueConsumer) {
        BronKerboschPivot.explore(graph, cliqueConsumer, PivotChoice.MaxDegreeLocal);
    }
}
