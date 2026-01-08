// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.Set;
import java.util.function.Consumer;
import java.util.stream.Collectors;

enum BronKerboschOrder {
    ;

    public static void explore(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer,
                               final PivotChoice furtherPivotChoice) {
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        final Set<Integer> mut_excluded = new HashSet<>(graph.order());
        final Iterable<Integer> vertices = () -> new DegeneracyOrdering(graph, -1);
        for (final var v : vertices) {
            final var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            final var neighbouringExcluded = util.Intersect(neighbours, mut_excluded)
                    .collect(Collectors.toCollection(HashSet::new));
            if (neighbouringExcluded.size() < neighbours.size()) {
                final var neighbouringCandidates = util.Difference(neighbours, neighbouringExcluded)
                        .collect(Collectors.toCollection(HashSet::new));
                BronKerboschPivot.visit(
                        graph, cliqueConsumer,
                        furtherPivotChoice,
                        neighbouringCandidates,
                        neighbouringExcluded,
                        new int[]{v}
                );
            }
            mut_excluded.add(v);
        }
    }
}
