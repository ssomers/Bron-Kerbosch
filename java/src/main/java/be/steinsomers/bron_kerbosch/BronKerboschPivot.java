package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashSet;
import java.util.Set;
import java.util.function.Consumer;
import java.util.stream.Collectors;
import java.util.stream.Stream;

class BronKerboschPivot implements BronKerboschAlgorithm {
    private final PivotChoice itsPivotChoice;

    BronKerboschPivot(PivotChoice pivotChoice) {
        itsPivotChoice = pivotChoice;
    }

    @Override
    public final Stream<int[]> explore(UndirectedGraph graph) {
        Stream.Builder<int[]> cliqueStream = Stream.builder();
        var order = graph.order();
        if (order > 0) {
            var pivot = graph.maxDegreeVertex();
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            Set<Integer> mut_excluded = new HashSet<>(order);
            for (int v = 0; v < order; ++v) {
                var neighbours = graph.neighbours(v);
                if (!neighbours.contains(pivot)) {
                    var neighbouringExcluded = util.Intersect(neighbours, mut_excluded)
                            .collect(Collectors.toCollection(HashSet::new));
                    if (neighbouringExcluded.size() < neighbours.size()) {
                        var neighbouringCandidates = util.Difference(neighbours,
                                        neighbouringExcluded)
                                .collect(Collectors.toCollection(HashSet::new));
                        visit(graph, cliqueStream,
                                itsPivotChoice,
                                neighbouringCandidates,
                                neighbouringExcluded,
                                new int[]{v});
                    }
                    mut_excluded.add(v);
                }
            }
        }
        return cliqueStream.build();
    }

    public static void visit(
            UndirectedGraph graph,
            Consumer<int[]> cliqueConsumer,
            PivotChoice pivotChoice,
            Set<Integer> mut_candidates,
            Set<Integer> mut_excluded,
            int[] cliqueInProgress
    ) {
        assert mut_candidates.stream().allMatch(v -> graph.degree(v) > 0);
        assert mut_excluded.stream().allMatch(v -> graph.degree(v) > 0);
        assert util.AreDisjoint(mut_candidates, mut_excluded);
        assert mut_candidates.size() >= 1;
        if (mut_candidates.size() == 1) {
            // Same logic as below, stripped down for this common case
            var v = mut_candidates.iterator().next();
            var neighbours = graph.neighbours(v);
            if (util.AreDisjoint(neighbours, mut_excluded)) {
                cliqueConsumer.accept(util.Append(cliqueInProgress, v));
            }
        } else {
            Collection<Integer> remainingCandidates;
            int pivot = -1;
            if (pivotChoice == PivotChoice.Arbitrary) {
                remainingCandidates = new ArrayList<>(mut_candidates);
                pivot = mut_candidates.iterator().next();
            } else {
                // Quickly handle locally unconnected candidates while finding pivot
                remainingCandidates = new ArrayList<>(mut_candidates.size());
                long seenLocalDegree = 0;
                for (var v : mut_candidates) {
                    var neighbours = graph.neighbours(v);
                    long localDegree = util.Intersect(neighbours, mut_candidates).count();
                    if (localDegree == 0) {
                        // Same logic as below, stripped down
                        if (util.AreDisjoint(neighbours, mut_excluded)) {
                            cliqueConsumer.accept(util.Append(cliqueInProgress, v));
                        }
                    } else {
                        if (seenLocalDegree < localDegree) {
                            seenLocalDegree = localDegree;
                            pivot = v;
                        }
                        remainingCandidates.add(v);
                    }
                }
                if (pivotChoice == PivotChoice.MaxDegreeLocalX && !remainingCandidates.isEmpty()) {
                    for (var v : mut_excluded) {
                        var neighbours = graph.neighbours(v);
                        var localDegree = util.Intersect(neighbours, mut_candidates).count();
                        if (seenLocalDegree < localDegree) {
                            seenLocalDegree = localDegree;
                            pivot = v;
                        }
                    }
                }
            }
            for (var v : remainingCandidates) {
                var neighbours = graph.neighbours(v);
                if (!neighbours.contains(pivot)) {
                    mut_candidates.remove(v);
                    var neighbouringCandidates = util.Intersect(neighbours, mut_candidates)
                            .collect(Collectors.toCollection(HashSet::new));
                    if (!neighbouringCandidates.isEmpty()) {
                        var neighbouringExcluded = util.Intersect(neighbours, mut_excluded)
                                .collect(Collectors.toCollection(HashSet::new));
                        visit(graph, cliqueConsumer,
                                pivotChoice,
                                neighbouringCandidates,
                                neighbouringExcluded,
                                util.Append(cliqueInProgress, v));
                    } else if (util.AreDisjoint(neighbours, mut_excluded)) {
                        cliqueConsumer.accept(util.Append(cliqueInProgress, v));
                    }
                    mut_excluded.add(v);
                }
            }
        }
    }
}
