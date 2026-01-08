// Naive Bron-Kerbosch algorithm

package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.Set;
import java.util.function.Consumer;
import java.util.stream.Collectors;

public final class BronKerbosch1 implements BronKerboschAlgorithm {
    @Override
    public void explore(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer) {
        final Set<Integer> candidates = graph.connectedVertices()
                .collect(Collectors.toCollection(HashSet::new));
        if (!candidates.isEmpty()) {
            final Set<Integer> excluded = new HashSet<>(candidates.size());
            visit(graph, cliqueConsumer, candidates, excluded, EMPTY_CLIQUE);
        }
    }

    private static void visit(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer,
                              final Set<Integer> mut_candidates, final Set<Integer> mut_excluded,
                              final int[] cliqueInProgress) {
        assert mut_candidates.stream().allMatch(graph::hasDegree);
        assert mut_excluded.stream().allMatch(graph::hasDegree);
        assert util.AreDisjoint(mut_candidates, mut_excluded);
        assert !mut_candidates.isEmpty();
        while (!mut_candidates.isEmpty()) {
            final var v = util.PopArbitrary(mut_candidates);
            final var neighbours = graph.neighbours(v);
            final var neighbouringCandidates = util.Intersect(mut_candidates, neighbours)
                    .collect(Collectors.toCollection(HashSet::new));
            if (!neighbouringCandidates.isEmpty()) {
                final var neighbouringExcluded = util.Intersect(mut_excluded, neighbours)
                        .collect(Collectors.toCollection(HashSet::new));
                visit(graph, cliqueConsumer,
                        neighbouringCandidates,
                        neighbouringExcluded,
                        util.Append(cliqueInProgress, v));
            } else if (util.AreDisjoint(mut_excluded, neighbours)) {
                cliqueConsumer.accept(util.Append(cliqueInProgress, v));
            }
            mut_excluded.add(v);
        }
    }
}
