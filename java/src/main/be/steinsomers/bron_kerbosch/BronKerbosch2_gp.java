package be.steinsomers.bron_kerbosch;

import java.util.stream.Stream;

public final class BronKerbosch2_gp implements BronKerboschAlgorithm {
    @Override
    public Stream<int[]> explore(UndirectedGraph graph) {
        return BronKerboschPivot.explore(graph, PivotChoice.MaxDegreeLocal);
    }
}
