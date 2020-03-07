package be.steinsomers.bron_kerbosch;
// Bron-Kerbosch algorithm with degeneracy ordering

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

class BronKerboschOrder implements BronKerboschAlgorithm {
    private final PivotChoice itsPivotChoice;
    private final boolean itsParallel;

    BronKerboschOrder(PivotChoice furtherPivotChoice) {
        itsPivotChoice = furtherPivotChoice;
        itsParallel = false;
    }

    BronKerboschOrder(PivotChoice furtherPivotChoice, boolean parallel) {
        itsPivotChoice = furtherPivotChoice;
        itsParallel = parallel;
    }

    @Override
    public final Stream<int[]> explore(UndirectedGraph graph) {
        Set<Integer> mut_excluded = new HashSet<>(graph.order());
        var vertices = new DegeneracyOrdering(graph, -1);
        var spliterator = new BronKerboschSpliterator(0);
        while (vertices.hasNext()) {
            var v = vertices.next();
            var neighbours = graph.neighbours(v);
            var neighbouringCandidates = util.Difference(neighbours, mut_excluded)
                    .collect(Collectors.toCollection(HashSet::new));
            if (neighbouringCandidates.isEmpty()) {
                assert !util.AreDisjoint(neighbours, mut_excluded);
            } else {
                var neighbouringExcluded = util.Intersect(neighbours, mut_excluded)
                        .collect(Collectors.toCollection(HashSet::new));
                var subWorker = new BronKerboschPivot.Worker(
                        graph,
                        itsPivotChoice,
                        itsPivotChoice,
                        neighbouringCandidates,
                        neighbouringExcluded);
                spliterator.offerInitial(v, subWorker);
            }
            mut_excluded.add(v);
        }
        return StreamSupport.stream(spliterator, itsParallel);
    }
}
