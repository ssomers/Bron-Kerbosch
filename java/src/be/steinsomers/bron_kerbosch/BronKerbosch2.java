package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.Set;

public final class BronKerbosch2 implements BronKerboschAlgorithm {
    public void explore(UndirectedGraph graph, Reporter reporter) {
        var candidates = graph.connectedVertices();
        if (!candidates.isEmpty()) {
            BronKerboschPivot.visit(
                    graph, reporter,
                    BronKerboschPivot.PivotChoice.Arbitrary,
                    BronKerboschPivot.PivotChoice.Arbitrary,
                    candidates,
                    Set.of(),
                    new ArrayList<>(candidates.size()));
        }
    }
}
