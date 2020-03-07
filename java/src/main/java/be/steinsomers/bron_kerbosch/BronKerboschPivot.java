package be.steinsomers.bron_kerbosch;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collection;
import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

class BronKerboschPivot implements BronKerboschAlgorithm {
    private final PivotChoice itsInitialPivotChoice;
    private final PivotChoice itsFurtherPivotChoice;

    BronKerboschPivot(PivotChoice initialPivotChoice,
                      PivotChoice furtherPivotChoice) {
        itsInitialPivotChoice = initialPivotChoice;
        itsFurtherPivotChoice = furtherPivotChoice;
    }

    @Override
    public final Collection<int[]> explore(UndirectedGraph graph) {
        Set<Integer> candidates = graph.connectedVertices()
                .collect(Collectors.toCollection(HashSet::new));
        Collection<int[]> cliques = new ArrayDeque<>();
        if (!candidates.isEmpty()) {
            Set<Integer> excluded = new HashSet<>(candidates.size());
            visit(
                    graph, cliques,
                    itsInitialPivotChoice,
                    itsFurtherPivotChoice,
                    candidates, excluded,
                    EMPTY_CLIQUE);
        }
        return cliques;
    }

    public static void visit(
            UndirectedGraph graph,
            Collection<int[]> mut_cliques,
            PivotChoice initialPivotChoice,
            PivotChoice furtherPivotChoice,
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
                mut_cliques.add(util.Append(cliqueInProgress, v));
            }
        } else {
            Collection<Integer> remainingCandidates;
            int pivot = -1;
            switch (initialPivotChoice) {
                case Arbitrary:
                    remainingCandidates = new ArrayList<>(mut_candidates);
                    pivot = mut_candidates.iterator().next();
                    break;
                case MaxDegree:
                    remainingCandidates = new ArrayList<>(mut_candidates);
                    pivot = mut_candidates.stream()
                            .max((v, w) -> graph.degree(v) > graph.degree(w) ? v : w)
                            .orElseThrow();
                    break;
                case MaxDegreeLocal:
                case MaxDegreeLocalX: {
                    // Quickly handle locally unconnected candidates while finding pivot
                    remainingCandidates = new ArrayList<>(mut_candidates.size());
                    long seenLocalDegree = 0;
                    for (var v : mut_candidates) {
                        var neighbours = graph.neighbours(v);
                        long localDegree = util.Intersect(neighbours, mut_candidates).count();
                        if (localDegree == 0) {
                            // Same logic as below, stripped down
                            if (util.AreDisjoint(neighbours, mut_excluded)) {
                                mut_cliques.add(util.Append(cliqueInProgress, v));
                            }
                        } else {
                            if (seenLocalDegree < localDegree) {
                                seenLocalDegree = localDegree;
                                pivot = v;
                            }
                            remainingCandidates.add(v);
                        }
                    }
                    if (initialPivotChoice == PivotChoice.MaxDegreeLocalX
                            && !remainingCandidates.isEmpty()) {
                        for (var v : mut_excluded) {
                            var neighbours = graph.neighbours(v);
                            var localDegree = util.Intersect(neighbours, mut_candidates).count();
                            if (seenLocalDegree < localDegree) {
                                seenLocalDegree = localDegree;
                                pivot = v;
                            }
                        }
                    }
                    break;
                }
                default:
                    throw new IndexOutOfBoundsException(initialPivotChoice.toString());
            }
            for (int v : remainingCandidates) {
                var neighbours = graph.neighbours(v);
                if (!neighbours.contains(pivot)) {
                    mut_candidates.remove(v);
                    var neighbouringCandidates = util.Intersect(mut_candidates, neighbours)
                            .collect(Collectors.toCollection(HashSet::new));
                    if (neighbouringCandidates.isEmpty()) {
                        if (util.AreDisjoint(neighbours, mut_excluded)) {
                            mut_cliques.add(util.Append(cliqueInProgress, v));
                        }
                    } else {
                        var neighbouringExcluded = util.Intersect(mut_excluded, neighbours)
                                .collect(Collectors.toCollection(HashSet::new));
                        visit(
                                graph, mut_cliques,
                                furtherPivotChoice,
                                furtherPivotChoice,
                                neighbouringCandidates,
                                neighbouringExcluded,
                                util.Append(cliqueInProgress, v)
                        );
                    }
                    mut_excluded.add(v);
                }
            }
        }
    }
}
