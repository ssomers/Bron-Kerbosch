package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

public class BronKerboschPivot implements BronKerboschAlgorithm {
    enum PivotChoice {
        Arbitrary, MaxDegree, MaxDegreeLocal, MaxDegreeLocalX
    }

    private final PivotChoice itsInitialPivotChoice;
    final PivotChoice itsFurtherPivotChoice;

    BronKerboschPivot(PivotChoice initial_pivot_choice,
                      PivotChoice further_pivot_choice) {
        itsInitialPivotChoice = initial_pivot_choice;
        itsFurtherPivotChoice = further_pivot_choice;
    }

    @Override
    public void explore(UndirectedGraph graph, Reporter reporter) {
        Set<Integer> candidates = graph.connectedVertices().collect(Collectors.toCollection(HashSet::new));
        if (!candidates.isEmpty()) {
            Set<Integer> excluded = new HashSet<>();
            visit(
                    graph, reporter,
                    itsInitialPivotChoice,
                    candidates, excluded,
                    List.of());
        }
    }

    void visit(
            UndirectedGraph graph,
            Reporter reporter,
            PivotChoice initial_pivot_choice,
            Set<Integer> mut_candidates,
            Set<Integer> mut_excluded,
            List<Integer> clique_in_progress
    ) {
        assert mut_candidates.size() > 0;
        assert mut_candidates.stream().allMatch(v -> graph.degree(v) > 0);
        assert mut_excluded.stream().allMatch(v -> graph.degree(v) > 0);
        assert util.AreDisjoint(mut_candidates, mut_excluded);

        if (mut_candidates.size() == 1) {
            // Same logic as below, stripped down for this common case
            var v = mut_candidates.iterator().next();
            var neighbours = graph.neighbours(v);
            if (util.AreDisjoint(neighbours, mut_excluded)) {
                reporter.record(util.Append(clique_in_progress, v));
            }
        } else {
            ArrayList<Integer> remaining_candidates;
            int pivot = -1;
            switch (initial_pivot_choice) {
                case Arbitrary:
                    remaining_candidates = new ArrayList<>(mut_candidates);
                    pivot = mut_candidates.iterator().next();
                    break;
                case MaxDegree:
                    remaining_candidates = new ArrayList<>(mut_candidates);
                    pivot = mut_candidates.stream()
                            .max((v, w) -> graph.degree(v) > graph.degree(w) ? v : w)
                            .orElseThrow();
                    break;
                case MaxDegreeLocal:
                case MaxDegreeLocalX: {
                    // Quickly handle locally unconnected candidates while finding pivot
                    remaining_candidates = new ArrayList<>(mut_candidates.size());
                    long seen_local_degree = 0;
                    for (var v : mut_candidates) {
                        var neighbours = graph.neighbours(v);
                        long local_degree = util.Intersect(neighbours, mut_candidates).count();
                        if (local_degree == 0) {
                            // Same logic as below, stripped down
                            if (util.AreDisjoint(neighbours, mut_excluded)) {
                                reporter.record(util.Append(clique_in_progress, v));
                            }
                        } else {
                            if (seen_local_degree < local_degree) {
                                seen_local_degree = local_degree;
                                pivot = v;
                            }
                            remaining_candidates.add(v);
                        }
                    }
                    if (seen_local_degree == 0) {
                        return;
                    }
                    if (initial_pivot_choice == PivotChoice.MaxDegreeLocalX) {
                        for (var v : mut_excluded) {
                            var neighbours = graph.neighbours(v);
                            var local_degree = util.Intersect(neighbours, mut_candidates).count();
                            if (seen_local_degree < local_degree) {
                                seen_local_degree = local_degree;
                                pivot = v;
                            }
                        }
                    }
                    break;
                }
                default:
                    throw new IndexOutOfBoundsException();
            }
            assert !remaining_candidates.isEmpty();
            for (int v : remaining_candidates) {
                var neighbours = graph.neighbours(v);
                if (!neighbours.contains(pivot)) {
                    mut_candidates.remove(v);
                    var neighbouring_candidates = util.Intersect(mut_candidates, neighbours)
                            .collect(Collectors.toCollection(HashSet::new));
                    if (neighbouring_candidates.isEmpty()) {
                        if (util.AreDisjoint(neighbours, mut_excluded)) {
                            reporter.record(util.Append(clique_in_progress, v));
                        }
                    } else {
                        var neighbouring_excluded = util.Intersect(mut_excluded, neighbours)
                                .collect(Collectors.toCollection(HashSet::new));
                        visit(
                                graph, reporter,
                                itsFurtherPivotChoice,
                                neighbouring_candidates,
                                neighbouring_excluded,
                                util.Append(clique_in_progress, v)
                        );
                    }
                    mut_excluded.add(v);
                }
            }
        }
    }
}
