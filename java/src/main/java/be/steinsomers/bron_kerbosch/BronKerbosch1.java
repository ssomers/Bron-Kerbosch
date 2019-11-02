// Naive Bron-Kerbosch algorithm

package be.steinsomers.bron_kerbosch;

import java.util.Collection;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

public final class BronKerbosch1 implements BronKerboschAlgorithm {
    @Override
    public void explore(UndirectedGraph graph, Reporter reporter) {
        Set<Integer> candidates = graph.connectedVertices()
                .collect(Collectors.toCollection(HashSet::new));
        Set<Integer> excluded = new HashSet<>(candidates.size());
        visit(graph, reporter, candidates, excluded, List.of());
    }

    private static void visit(UndirectedGraph graph, Reporter reporter,
                              Set<Integer> mut_candidates, Set<Integer> mut_excluded,
                              Collection<Integer> cliqueInProgress) {
        while (!mut_candidates.isEmpty()) {
            var v = util.PopArbitrary(mut_candidates);
            var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            var neighbouringCandidates = util.Intersect(mut_candidates, neighbours)
                    .collect(Collectors.toCollection(HashSet::new));
            if (neighbouringCandidates.isEmpty()) {
                if (util.AreDisjoint(mut_excluded, neighbours))
                    reporter.record(util.Append(cliqueInProgress, v));
            } else {
                var neighbouringExcluded = util.Intersect(mut_excluded, neighbours)
                        .collect(Collectors.toCollection(HashSet::new));
                visit(
                        graph, reporter,
                        neighbouringCandidates,
                        neighbouringExcluded,
                        util.Append(cliqueInProgress, v));
            }
            mut_excluded.add(v);
        }
    }
}
