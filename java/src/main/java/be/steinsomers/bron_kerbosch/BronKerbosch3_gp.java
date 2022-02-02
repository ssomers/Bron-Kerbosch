package be.steinsomers.bron_kerbosch;

import java.util.stream.Stream;

public final class BronKerbosch3_gp implements BronKerboschAlgorithm {
    @Override
    public Stream<int[]> explore(UndirectedGraph graph) {
        return BronKerboschOrder.explore(graph, PivotChoice.MaxDegreeLocal);
    }
}
