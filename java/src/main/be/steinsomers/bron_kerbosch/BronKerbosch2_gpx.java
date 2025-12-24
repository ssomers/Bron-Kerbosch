package be.steinsomers.bron_kerbosch;

import java.util.stream.Stream;

public final class BronKerbosch2_gpx implements BronKerboschAlgorithm {
    @Override
    public Stream<int[]> explore(UndirectedGraph graph) {
        return BronKerboschPivot.explore(graph, PivotChoice.MaxDegreeLocalX);
    }
}
