package be.steinsomers.bron_kerbosch;

import java.util.stream.Stream;

public final class BronKerbosch3_gpx implements BronKerboschAlgorithm {
    @Override
    public Stream<int[]> explore(UndirectedGraph graph) {
        return BronKerboschOrder.explore(graph, PivotChoice.MaxDegreeLocalX);
    }
}
