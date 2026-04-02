// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.stream.Collectors;

enum BronKerboschDegeneracy {
    ;

    public static void explore(final UndirectedGraph graph, final CliqueConsumer cliqueConsumer,
                               final PivotChoice furtherPivotChoice) {
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        final var excluded = new boolean[graph.order()];
        final Iterable<Integer> vertices = () -> new DegeneracyIterator(graph);
        for (final var v : vertices) {
            final var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            final var neighbouringExcluded = util.Intersect(neighbours, excluded)
                    .collect(Collectors.toCollection(HashSet::new));
            final var neighbouringCandidates = util.Difference(neighbours, neighbouringExcluded)
                    .collect(Collectors.toCollection(HashSet::new));
            BronKerboschPivot.visit(
                    graph, cliqueConsumer,
                    furtherPivotChoice,
                    neighbouringCandidates,
                    neighbouringExcluded,
                    new int[]{v}
            );
            excluded[v] = true;
        }
    }
}
