package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashSet;
import java.util.Set;
import java.util.function.Consumer;
import java.util.stream.Collectors;

enum BronKerboschPivot {
    ;

    public static void explore(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer,
                               final PivotChoice pivotChoice) {
        final var order = graph.order();
        if (order > 0) {
            final var pivot = graph.maxDegreeVertex();
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            final Set<Integer> mut_excluded = new HashSet<>(order);
            for (int v = 0; v < order; ++v) {
                final var neighbours = graph.neighbours(v);
                if (!neighbours.contains(pivot)) {
                    final var neighbouringExcluded = util.Intersect(neighbours, mut_excluded)
                            .collect(Collectors.toCollection(HashSet::new));
                    if (neighbouringExcluded.size() < neighbours.size()) {
                        final var neighbouringCandidates = util.Difference(neighbours, neighbouringExcluded)
                                .collect(Collectors.toCollection(HashSet::new));
                        visit(graph, cliqueConsumer, pivotChoice, neighbouringCandidates, neighbouringExcluded,
                                new int[]{v});
                    }
                    mut_excluded.add(v);
                }
            }
        }
    }

    public static void visit(
            final UndirectedGraph graph,
            final Consumer<int[]> cliqueConsumer,
            final PivotChoice pivotChoice,
            final Set<Integer> mut_candidates,
            final Set<Integer> mut_excluded,
            final int[] cliqueInProgress
    ) {
        assert mut_candidates.stream().allMatch(graph::hasDegree);
        assert mut_excluded.stream().allMatch(graph::hasDegree);
        assert util.AreDisjoint(mut_candidates, mut_excluded);
        //noinspection SizeReplaceableByIsEmpty
        assert mut_candidates.size() >= 1;
        if (mut_candidates.size() == 1) {
            // Same logic as below, stripped down for this common case
            final var v = mut_candidates.iterator().next();
            final var neighbours = graph.neighbours(v);
            if (util.AreDisjoint(neighbours, mut_excluded)) {
                cliqueConsumer.accept(util.Append(cliqueInProgress, v));
            }
        } else if (pivotChoice == PivotChoice.Arbitrary) {
            final var remainingCandidates = new ArrayList<>(mut_candidates);
            final int pivot = remainingCandidates.getFirst();
            visitAroundPivot(graph, cliqueConsumer, mut_candidates, mut_excluded, cliqueInProgress,
                    PivotChoice.Arbitrary, pivot, remainingCandidates);
        } else {
            visitMaxDegree(graph, cliqueConsumer, mut_candidates, mut_excluded, cliqueInProgress, pivotChoice);
        }
    }

    private static void visitMaxDegree(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer,
                                       final Set<Integer> mut_candidates, final Set<Integer> mut_excluded,
                                       final int[] cliqueInProgress, final PivotChoice pivotChoice) {
        assert pivotChoice == PivotChoice.MaxDegreeLocal || pivotChoice == PivotChoice.MaxDegreeLocalX;
        // Quickly handle locally unconnected candidates while finding pivot
        int pivot = -1;
        final Collection<Integer> remainingCandidates = new ArrayList<>(mut_candidates.size());
        long seenLocalDegree = 0;
        for (final var v : mut_candidates) {
            final var neighbours = graph.neighbours(v);
            final long localDegree = util.Intersect(neighbours, mut_candidates).count();
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
            for (final var v : mut_excluded) {
                final var neighbours = graph.neighbours(v);
                final var localDegree = util.Intersect(neighbours, mut_candidates).count();
                if (seenLocalDegree < localDegree) {
                    seenLocalDegree = localDegree;
                    pivot = v;
                }
            }
        }
        visitAroundPivot(graph, cliqueConsumer, mut_candidates, mut_excluded, cliqueInProgress,
                pivotChoice, pivot, remainingCandidates);
    }

    private static void visitAroundPivot(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer,
                                         final Set<Integer> mut_candidates, final Set<Integer> mut_excluded,
                                         final int[] cliqueInProgress, final PivotChoice furtherPivotChoice,
                                         final int pivot, final Iterable<Integer> remainingCandidates) {
        for (final var v : remainingCandidates) {
            final var neighbours = graph.neighbours(v);
            if (!neighbours.contains(pivot)) {
                mut_candidates.remove(v);
                final var neighbouringCandidates = util.Intersect(neighbours, mut_candidates)
                        .collect(Collectors.toCollection(HashSet::new));
                if (!neighbouringCandidates.isEmpty()) {
                    final var neighbouringExcluded = util.Intersect(neighbours, mut_excluded)
                            .collect(Collectors.toCollection(HashSet::new));
                    visit(graph, cliqueConsumer, furtherPivotChoice,
                            neighbouringCandidates, neighbouringExcluded,
                            util.Append(cliqueInProgress, v));
                } else if (util.AreDisjoint(neighbours, mut_excluded)) {
                    cliqueConsumer.accept(util.Append(cliqueInProgress, v));
                }
                mut_excluded.add(v);
            }
        }
    }
}
