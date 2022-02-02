package be.steinsomers.bron_kerbosch;

import java.util.stream.Stream;

public final class BronKerbosch2 implements BronKerboschAlgorithm {
    @Override
    public Stream<int[]> explore(UndirectedGraph graph) {
        return BronKerboschPivot.explore(graph, PivotChoice.Arbitrary);
    }
}
