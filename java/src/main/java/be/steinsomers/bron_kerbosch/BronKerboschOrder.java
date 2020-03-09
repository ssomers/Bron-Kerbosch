package be.steinsomers.bron_kerbosch;
// Bron-Kerbosch algorithm with degeneracy ordering

import lombok.AllArgsConstructor;
import lombok.NonNull;

import java.util.Collection;
import java.util.HashSet;
import java.util.PrimitiveIterator;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

class BronKerboschOrder implements BronKerboschAlgorithm {
    private final PivotChoice itsPivotChoice;

    BronKerboschOrder(PivotChoice furtherPivotChoice) {
        itsPivotChoice = furtherPivotChoice;
    }

    @Override
    public final Stream<int[]> explore(UndirectedGraph graph) {
        Set<Integer> mut_excluded = new HashSet<>(graph.order());
        var vertices = new DegeneracyOrdering(graph, -1);
        var worker = new Worker(graph, itsPivotChoice, vertices, mut_excluded);
        var spliterator = new BronKerboschSpliterator(-1, worker);
        return StreamSupport.stream(spliterator, true);
    }

    @AllArgsConstructor
    private static final class Worker implements BronKerboschSpliterator.Generator {
        private final @NonNull UndirectedGraph graph;
        private final PivotChoice pivotChoice;
        private final PrimitiveIterator.OfInt vertices;
        private final Set<Integer> mut_excluded;

        public boolean findNextVertex(BronKerboschSpliterator.VtxConsumer consumer) {
            while (vertices.hasNext()) {
                var v = vertices.next();
                mut_excluded.add(v);
                var neighbours = graph.neighbours(v);
                assert !neighbours.isEmpty();
                var neighbouringCandidates = util.Difference(neighbours, mut_excluded)
                        .collect(Collectors.toCollection(HashSet::new));
                if (neighbouringCandidates.isEmpty()) {
                    assert !util.AreDisjoint(neighbours, mut_excluded);
                } else {
                    var neighbouringExcluded = util.Intersect(neighbours, mut_excluded)
                            .collect(Collectors.toCollection(HashSet::new));
                    var subWorker = new BronKerboschPivot.Worker(
                            graph,
                            pivotChoice,
                            pivotChoice,
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
