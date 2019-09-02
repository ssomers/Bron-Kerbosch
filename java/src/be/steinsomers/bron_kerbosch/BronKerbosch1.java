// Naive Bron-Kerbosch algorithm

package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

public final class BronKerbosch1 implements BronKerboschAlgorithm {
    @Override
    public void explore(UndirectedGraph graph, Reporter reporter) {
        Set<Integer> candidates = graph.connectedVertices().collect(Collectors.toCollection(HashSet::new));
        Set<Integer> excluded = new HashSet<>();
        visit(graph, reporter, candidates, excluded, List.of());
    }

    private void visit(UndirectedGraph graph, Reporter reporter,
                       Set<Integer> mut_candidates, Set<Integer> mut_excluded,
                       List<Integer> clique) {
        while (!mut_candidates.isEmpty()) {
            var v = util.PopArbitrary(mut_candidates);
            var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            var neighbouring_candidates = util.Intersect(mut_candidates, neighbours)
                    .collect(Collectors.toCollection(HashSet::new));
            if (!neighbouring_candidates.isEmpty()) {
                var neighbouring_excluded = util.Intersect(mut_excluded, neighbours)
                        .collect(Collectors.toCollection(HashSet::new));
                visit(
                        graph, reporter,
                        neighbouring_candidates,
                        neighbouring_excluded,
                        util.Append(clique, v));
            } else {
                if (util.AreDisjoint(mut_excluded, neighbours))
                    reporter.record(util.Append(clique, v));
            }
            mut_excluded.add(v);
        }
    }
}
