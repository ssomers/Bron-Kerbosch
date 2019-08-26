// Naive Bron-Kerbosch algorithm

package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public final class BronKerbosch1 {
    static public void explore(UndirectedGraph graph, Reporter reporter) {
        var candidates = graph.connectedVertices();
        visit(
                graph,
                reporter,
                candidates,
                new HashSet<Integer>(),
                new ArrayList<Integer>(candidates.size()));
    }


    static void visit(UndirectedGraph graph, Reporter reporter,
                      Set<Integer> candidates, Set<Integer> excluded, List<Integer> clique) {
        while (!candidates.isEmpty()) {
            var v = util.PopArbitrary(candidates);
            var neighbours = graph.neighbours(v);
            assert !neighbours.isEmpty();
            var neighbouring_candidates = util.Intersect(candidates, neighbours);
            if (!neighbouring_candidates.isEmpty()) {
                var neighbouring_excluded = util.Intersect(excluded, neighbours);
                visit(graph, reporter, neighbouring_candidates, neighbouring_excluded, util.Append(clique, v));
            } else {
                if (util.AreDisjoint(excluded, neighbours))
                    reporter.record(util.Append(clique, v));
            }
            excluded.add(v);
        }
    }
}
