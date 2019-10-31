package be.steinsomers.bron_kerbosch;
// Bron-Kerbosch algorithm with degeneracy ordering

import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

class BronKerboschOrder implements BronKerboschAlgorithm {
    private final PivotChoice itsPivotChoice;

    BronKerboschOrder(PivotChoice furtherPivotChoice) {
        itsPivotChoice = furtherPivotChoice;
    }

    @Override
    public final void explore(UndirectedGraph graph, Reporter reporter) {
        Set<Integer> mut_excluded = new HashSet<>(graph.order());
        Iterable<Integer> vertices = () -> new DegeneracyOrdering(graph, -1);
        for (var v : vertices) {
            var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            var neighbouringCandidates = util.Difference(neighbours, mut_excluded)
                    .collect(Collectors.toCollection(HashSet::new));
            if (neighbouringCandidates.isEmpty()) {
                assert !util.AreDisjoint(neighbours, mut_excluded);
            } else {
                var neighbouringExcluded = util.Intersect(neighbours, mut_excluded)
                        .collect(Collectors.toCollection(HashSet::new));
                BronKerboschPivot.visit(
                        graph, reporter,
                        itsPivotChoice,
                        itsPivotChoice,
                        neighbouringCandidates,
                        neighbouringExcluded,
                        List.of(v)
                );
            }
            mut_excluded.add(v);
        }
    }
}
