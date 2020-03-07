// Naive Bron-Kerbosch algorithm

package be.steinsomers.bron_kerbosch;

import lombok.NonNull;

import java.util.HashSet;
import java.util.Set;
import java.util.function.Consumer;
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
        var spliterator = new BronKerboschSpliterator(worker);
        return StreamSupport.stream(spliterator, false);
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

        public boolean findNextVertex(Consumer<Integer> cliqueConsumer,
                                      BronKerboschSpliterator.VertexVisitQueue recursionQueue) {
            while (!mut_candidates.isEmpty()) {
                var v = util.PopArbitrary(mut_candidates);
                var neighbours = graph.neighbours(v);
                assert !neighbours.isEmpty();
                var neighbouringCandidates = util.Intersect(mut_candidates, neighbours)
                        .collect(Collectors.toCollection(HashSet::new));
                if (neighbouringCandidates.isEmpty()) {
                    if (cliqueConsumer == null) {
                        assert !util.AreDisjoint(mut_excluded, neighbours);
                    } else if (util.AreDisjoint(mut_excluded, neighbours)) {
                        cliqueConsumer.accept(v);
                        return true;
                    }
                } else {
                    var neighbouringExcluded = util.Intersect(mut_excluded, neighbours)
                            .collect(Collectors.toCollection(HashSet::new));
                    var subWorker = new Worker(
                            graph,
                            neighbouringCandidates,
                            neighbouringExcluded);
                    recursionQueue.offer(v, subWorker);
                }
                mut_excluded.add(v);
            }
            return false;
        }
    }
}
