// Naive Bron-Kerbosch algorithm

package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.Set;
import java.util.function.Consumer;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public final class BronKerbosch1 implements BronKerboschAlgorithm {
    @Override
    public Stream<int[]> explore(UndirectedGraph graph) {
        Stream.Builder<int[]> cliqueStream = Stream.builder();
        Set<Integer> candidates = graph.connectedVertices()
                .collect(Collectors.toCollection(HashSet::new));
        if (!candidates.isEmpty()) {
            Set<Integer> excluded = new HashSet<>(candidates.size());
            visit(graph, cliqueStream, candidates, excluded, EMPTY_CLIQUE);
        }
        return cliqueStream.build();
    }

    private static void visit(UndirectedGraph graph, Consumer<int[]> cliqueConsumer,
                              Set<Integer> mut_candidates, Set<Integer> mut_excluded,
                              int[] cliqueInProgress) {
        assert mut_candidates.stream().allMatch(graph::hasDegree);
        assert mut_excluded.stream().allMatch(graph::hasDegree);
        assert util.AreDisjoint(mut_candidates, mut_excluded);
        assert !mut_candidates.isEmpty();
        while (!mut_candidates.isEmpty()) {
            var v = util.PopArbitrary(mut_candidates);
            var neighbours = graph.neighbours(v);
            var neighbouringCandidates = util.Intersect(mut_candidates, neighbours)
                    .collect(Collectors.toCollection(HashSet::new));
            if (!neighbouringCandidates.isEmpty()) {
                var neighbouringExcluded = util.Intersect(mut_excluded, neighbours)
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
