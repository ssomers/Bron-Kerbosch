// Naive Bron-Kerbosch algorithm

package be.steinsomers.bron_kerbosch;

import lombok.NonNull;

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public final class BronKerbosch1 implements BronKerboschAlgorithm {
    @Override
    public Stream<int[]> explore(UndirectedGraph graph) {
        Set<Integer> candidates = graph.connectedVertices()
                .collect(Collectors.toCollection(HashSet::new));
        Set<Integer> excluded = new HashSet<>(candidates.size());
        var worker = new Worker(graph, candidates, excluded);
        var spliterator = new BronKerboschSpliterator(-1, worker);
        return StreamSupport.stream(spliterator, true);
    }

    private static final class Worker implements BronKerboschSpliterator.Generator {
        private final @NonNull UndirectedGraph graph;
        private final @NonNull Set<Integer> mut_candidates;
        private final @NonNull Set<Integer> mut_excluded;

        Worker(final @NonNull UndirectedGraph graph,
               @NonNull Set<Integer> candidates,
               @NonNull Set<Integer> excluded) {
            this.graph = graph;
            mut_candidates = candidates;
            mut_excluded = excluded;
        }

        public boolean findNextVertex(BronKerboschSpliterator.VtxConsumer consumer) {
            while (!mut_candidates.isEmpty()) {
                var v = util.PopArbitrary(mut_candidates);
                mut_excluded.add(v);
                var neighbours = graph.neighbours(v);
                assert !neighbours.isEmpty();
                var neighbouringCandidates = util.Intersect(mut_candidates, neighbours)
                        .collect(Collectors.toCollection(HashSet::new));
                if (neighbouringCandidates.isEmpty()) {
                    if (util.AreDisjoint(mut_excluded, neighbours)) {
                        consumer.acceptClique(v);
                        return true;
                    }
                } else {
                    var neighbouringExcluded = util.Intersect(mut_excluded, neighbours)
                            .collect(Collectors.toCollection(HashSet::new));
                    var subWorker = new Worker(
                            graph,
                            neighbouringCandidates,
                            neighbouringExcluded);
                    consumer.diveDeeper(v, subWorker);
                    return true;
                }
            }
            return false;
        }
    }
}
