// Naive Bron-Kerbosch algorithm

package be.steinsomers.bron_kerbosch;

import java.util.ArrayDeque;
import java.util.Collection;
import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

public final class BronKerbosch1 implements BronKerboschAlgorithm {
    @Override
    public Collection<int[]> explore(UndirectedGraph graph) {
        Set<Integer> candidates = graph.connectedVertices()
                .collect(Collectors.toCollection(HashSet::new));
        Set<Integer> excluded = new HashSet<>(candidates.size());
        Collection<int[]> cliques = new ArrayDeque<>();
        visit(graph, cliques, candidates, excluded, EMPTY_CLIQUE);
        return cliques;
    }

    private static void visit(UndirectedGraph graph, Collection<int[]> mut_cliques,
                              Set<Integer> mut_candidates, Set<Integer> mut_excluded,
                              int[] cliqueInProgress) {
        while (!mut_candidates.isEmpty()) {
            var v = util.PopArbitrary(mut_candidates);
            var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            var neighbouringCandidates = util.Intersect(mut_candidates, neighbours)
                    .collect(Collectors.toCollection(HashSet::new));
            if (neighbouringCandidates.isEmpty()) {
                if (util.AreDisjoint(mut_excluded, neighbours))
                    mut_cliques.add(util.Append(cliqueInProgress, v));
            } else {
                var neighbouringExcluded = util.Intersect(mut_excluded, neighbours)
                        .collect(Collectors.toCollection(HashSet::new));
                visit(
                        graph, mut_cliques,
                        neighbouringCandidates,
                        neighbouringExcluded,
                        util.Append(cliqueInProgress, v));
            }
            mut_excluded.add(v);
        }
    }
}
