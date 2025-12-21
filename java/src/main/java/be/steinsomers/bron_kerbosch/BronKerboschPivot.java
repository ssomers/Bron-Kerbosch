package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashSet;
import java.util.Set;
import java.util.function.Consumer;
import java.util.stream.Collectors;
import java.util.stream.Stream;

final class BronKerboschPivot {
    public static Stream<int[]> explore(UndirectedGraph graph, PivotChoice pivotChoice) {
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
                        visit(graph, cliqueStream, pivotChoice,
                                neighbouringCandidates, neighbouringExcluded, new int[]{v});
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
        assert mut_candidates.stream().allMatch(graph::hasDegree);
        assert mut_excluded.stream().allMatch(graph::hasDegree);
        assert util.AreDisjoint(mut_candidates, mut_excluded);
        //noinspection SizeReplaceableByIsEmpty
        assert mut_candidates.size() >= 1;
        if (mut_candidates.size() == 1) {
            // Same logic as below, stripped down for this common case
            var v = mut_candidates.iterator().next();
            var neighbours = graph.neighbours(v);
            if (util.AreDisjoint(neighbours, mut_excluded)) {
                cliqueConsumer.accept(util.Append(cliqueInProgress, v));
            }
        } else if (pivotChoice == PivotChoice.Arbitrary) {
            var remainingCandidates = new ArrayList<>(mut_candidates);
            int pivot = remainingCandidates.getFirst();
            visitAroundPivot(graph, cliqueConsumer, mut_candidates, mut_excluded, cliqueInProgress,
                    PivotChoice.Arbitrary, pivot, remainingCandidates);
        } else {
            visitMaxDegree(graph, cliqueConsumer, mut_candidates, mut_excluded, cliqueInProgress,
                    pivotChoice);
        }
    }

    private static void visitMaxDegree(UndirectedGraph graph, Consumer<int[]> cliqueConsumer,
                                       Set<Integer> mut_candidates, Set<Integer> mut_excluded,
                                       int[] cliqueInProgress, PivotChoice pivotChoice) {
        assert pivotChoice == PivotChoice.MaxDegreeLocal ||
                pivotChoice == PivotChoice.MaxDegreeLocalX;
        // Quickly handle locally unconnected candidates while finding pivot
        int pivot = -1;
        Collection<Integer> remainingCandidates = new ArrayList<>(mut_candidates.size());
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
        visitAroundPivot(graph, cliqueConsumer, mut_candidates, mut_excluded, cliqueInProgress,
                pivotChoice, pivot, remainingCandidates);
    }

    private static void visitAroundPivot(UndirectedGraph graph, Consumer<int[]> cliqueConsumer,
                                         Set<Integer> mut_candidates, Set<Integer> mut_excluded,
                                         int[] cliqueInProgress, PivotChoice furtherPivotChoice,
                                         int pivot, Iterable<Integer> remainingCandidates) {
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
                            furtherPivotChoice,
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
