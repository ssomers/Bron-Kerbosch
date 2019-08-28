package be.steinsomers.bron_kerbosch;

import java.util.*;
import java.util.stream.Collectors;

public class BronKerboschPivot implements BronKerboschAlgorithm {
    enum PivotChoice {
        Arbitrary, MaxDegree, MaxDegreeLocal, MaxDegreeLocalX
    }

    private final PivotChoice itsInitialPivotChoice;
    private final PivotChoice itsFurtherPivotChoice;

    BronKerboschPivot(PivotChoice initial_pivot_choice,
                      PivotChoice further_pivot_choice) {
        itsInitialPivotChoice = initial_pivot_choice;
        itsFurtherPivotChoice = further_pivot_choice;
    }

    @Override
    public void explore(UndirectedGraph graph, Reporter reporter) {
        var candidates = graph.connectedVertices().collect(Collectors.toCollection(HashSet::new));
        if (!candidates.isEmpty()) {
            visit(
                    graph, reporter,
                    itsInitialPivotChoice,
                    candidates,
                    Set.of(),
                    new ArrayList<>(candidates.size()));
        }
    }

    private void visit(
            UndirectedGraph graph,
            Reporter reporter,
            PivotChoice initial_pivot_choice,
            Set<Integer> candidates,
            Set<Integer> excluded,
            List<Integer> clique_in_progress
    ) {
        assert !candidates.isEmpty();
        assert candidates.stream().allMatch((Integer v) -> graph.degree(v) > 0);
        assert excluded.stream().allMatch((Integer v) -> graph.degree(v) > 0);
        assert util.AreDisjoint(candidates, excluded);

        if (candidates.size() == 1) {
            // Same logic as below, stripped down for this common case
            var v = candidates.iterator().next();
            var neighbours = graph.neighbours(v);
            if (util.AreDisjoint(neighbours, excluded)) {
                reporter.record(util.Append(clique_in_progress, v));
            }
        } else {
            Collection<Integer> remaining_candidates;
            int pivot = -1;
            switch (initial_pivot_choice) {
                case Arbitrary:
                    remaining_candidates = candidates;
                    pivot = candidates.iterator().next();
                    break;
                case MaxDegree:
                    remaining_candidates = candidates;
                    pivot = candidates.stream()
                            .max((Integer v, Integer w) -> graph.degree(v) > graph.degree(w) ? v : w)
                            .orElseThrow();
                    break;
                case MaxDegreeLocal:
                case MaxDegreeLocalX: {
                    // Quickly handle locally unconnected candidates while finding pivot
                    var remaining_candidate_list = new ArrayList<Integer>(candidates.size());
                    long seen_local_degree = 0;
                    for (var v : candidates) {
                        var neighbours = graph.neighbours(v);
                        var local_degree = util.intersection_size(neighbours, candidates);
                        if (local_degree == 0) {
                            // Same logic as below, stripped down
                            if (util.AreDisjoint(neighbours, excluded)) {
                                reporter.record(util.Append(clique_in_progress, v));
                            }
                        } else {
                            if (seen_local_degree < local_degree) {
                                seen_local_degree = local_degree;
                                pivot = v;
                            }
                            remaining_candidate_list.add(v);
                        }
                    }
                    if (seen_local_degree == 0) {
                        return;
                    }
                    if (initial_pivot_choice == PivotChoice.MaxDegreeLocalX) {
                        for (var v : excluded) {
                            var neighbours = graph.neighbours(v);
                            var local_degree = util.intersection_size(neighbours, candidates);
                            if (seen_local_degree < local_degree) {
                                seen_local_degree = local_degree;
                                pivot = v;
                            }
                        }
                    }
                    remaining_candidates = remaining_candidate_list;
                    break;
                }
                default:
                    throw new IndexOutOfBoundsException();
            }
            assert !remaining_candidates.isEmpty();
            var mut_candidates = new HashSet<>(candidates);
            var mut_excluded = new HashSet<>(excluded);
            for (int v : remaining_candidates) {
                var neighbours = graph.neighbours(v);
                if (!neighbours.contains(pivot)) {
                    mut_candidates.remove(v);
                    var neighbouring_candidates = util.Intersect(mut_candidates, neighbours);
                    if (neighbouring_candidates.isEmpty()) {
                        if (util.AreDisjoint(neighbours, mut_excluded)) {
                            reporter.record(util.Append(clique_in_progress, v));
                        }
                    } else {
                        var neighbouring_excluded = util.Intersect(mut_excluded, neighbours);
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
