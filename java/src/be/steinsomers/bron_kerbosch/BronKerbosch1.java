// Naive Bron-Kerbosch algorithm

package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.List;
import java.util.stream.Collectors;

public final class BronKerbosch1 implements BronKerboschAlgorithm {
    public void explore(UndirectedGraph graph, Reporter reporter) {
        var candidates = graph.connectedVertices().collect(Collectors.toCollection(HashSet::new));
        visit(graph, reporter, candidates, new HashSet<>(), List.of());
    }

    private void visit(UndirectedGraph graph, Reporter reporter,
                       HashSet<Integer> candidates, HashSet<Integer> excluded, List<Integer> clique) {
        while (!candidates.isEmpty()) {
            var v = util.PopArbitrary(candidates);
            var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            var neighbouring_candidates = util.Intersect(candidates, neighbours);
            if (!neighbouring_candidates.isEmpty()) {
                var neighbouring_excluded = util.Intersect(excluded, neighbours);
                visit(graph, reporter, neighbouring_candidates, neighbouring_excluded, util.Append(clique, v));
            } else {
                if (util.AreDisjoint(excluded, neighbours))
                    reporter.record(util.Append(clique, v));
            }
            excluded.add(v);
        }
    }
}
