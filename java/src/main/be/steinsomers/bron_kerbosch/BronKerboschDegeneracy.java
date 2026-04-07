// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.Set;

enum BronKerboschDegeneracy {
    ;

    public static void explore(final UndirectedGraph graph, final CliqueConsumer cliqueConsumer,
                               final PivotChoice furtherPivotChoice) {
        final var degeneracy = new DegeneracyIterator(graph);
        degeneracy.forEachRemaining((int v) -> {
            final var neighbours = graph.neighbours(v);
            final Set<Integer> neighbouringCandidates = new HashSet<>(neighbours.size());
            final Set<Integer> neighbouringExcluded = new HashSet<>(neighbours.size() - 1);
            neighbours.forEach((Integer w) -> (degeneracy.isCandidate(w)
                    ? neighbouringCandidates
                    : neighbouringExcluded).add(w));
            assert !neighbouringCandidates.isEmpty();
            BronKerboschPivot.visit(
                    graph, cliqueConsumer,
                    furtherPivotChoice,
                    neighbouringCandidates,
                    neighbouringExcluded,
                    new int[]{v}
            );
        });
    }
}
