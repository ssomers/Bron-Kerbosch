package be.steinsomers.bron_kerbosch;

public final class BronKerbosch3_gpx implements BronKerboschAlgorithm {
    @Override
    public void explore(final UndirectedGraph graph, final CliqueConsumer cliqueConsumer) {
        BronKerboschDegeneracy.explore(graph, cliqueConsumer, PivotChoice.MaxDegreeLocalX);
    }
}
