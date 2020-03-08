package be.steinsomers.bron_kerbosch;
// Bron-Kerbosch algorithm with degeneracy ordering

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

class BronKerboschOrder implements BronKerboschAlgorithm {
    private final PivotChoice itsPivotChoice;

    BronKerboschOrder(PivotChoice furtherPivotChoice) {
        itsPivotChoice = furtherPivotChoice;
    }

    @Override
    public final Stream<int[]> explore(UndirectedGraph graph) {
        Set<Integer> mut_excluded = new HashSet<>(graph.order());
        Stream.Builder<int[]> cliqueStream = Stream.builder();
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
                        graph, cliqueStream,
                        itsPivotChoice,
                        itsPivotChoice,
                        neighbouringCandidates,
                        neighbouringExcluded,
                        new int[]{v}
                );
            }
            mut_excluded.add(v);
        }
        return cliqueStream.build();
    }
}
