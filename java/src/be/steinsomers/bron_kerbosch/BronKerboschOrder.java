package be.steinsomers.bron_kerbosch;
// Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
// choosing a pivot arbitrarily

import java.util.HashSet;
import java.util.List;
import java.util.stream.Collectors;

public class BronKerboschOrder extends BronKerboschPivot {
    BronKerboschOrder(PivotChoice further_pivot_choice) {
        super(further_pivot_choice, further_pivot_choice);
    }

    @Override
    public void explore(UndirectedGraph graph, Reporter reporter) {
        HashSet<Integer> excluded = new HashSet<>();
        Iterable<Integer> vertices = () -> new DegeneracyOrdering(graph, -1);
        for (var v : vertices) {
            var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            var neighbouring_candidates = util.Difference(neighbours, excluded).collect(Collectors.toCollection(HashSet::new));
            if (!neighbouring_candidates.isEmpty()) {
                var neighbouring_excluded = util.Intersect(neighbours, excluded).collect(Collectors.toCollection(HashSet::new));
                visit(
                        graph, reporter,
                        itsFurtherPivotChoice,
                        neighbouring_candidates,
                        neighbouring_excluded,
                        List.of(v)
                );
            } else {
                assert !util.AreDisjoint(neighbours, excluded);
            }
            excluded.add(v);
        }
    }
}
