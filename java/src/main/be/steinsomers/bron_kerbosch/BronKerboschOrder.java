// Bron-Kerbosch algorithm with degeneracy ordering
package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

final class BronKerboschOrder {
    public static Stream<int[]> explore(UndirectedGraph graph, PivotChoice furtherPivotChoice) {
        Stream.Builder<int[]> cliqueStream = Stream.builder();
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        Set<Integer> mut_excluded = new HashSet<>(graph.order());
        Iterable<Integer> vertices = () -> new DegeneracyOrdering(graph, -1);
        for (var v : vertices) {
            var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            var neighbouringExcluded = util.Intersect(neighbours, mut_excluded)
                    .collect(Collectors.toCollection(HashSet::new));
            if (neighbouringExcluded.size() < neighbours.size()) {
                var neighbouringCandidates = util.Difference(neighbours, neighbouringExcluded)
                        .collect(Collectors.toCollection(HashSet::new));
                BronKerboschPivot.visit(
                        graph, cliqueStream,
                        furtherPivotChoice,
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
