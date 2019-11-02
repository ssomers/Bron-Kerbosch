package be.steinsomers.bron_kerbosch;

import lombok.AllArgsConstructor;
import lombok.Data;

import java.util.Collection;
import java.util.HashSet;
import java.util.List;
import java.util.Objects;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public final class BronKerbosch3_ST implements BronKerboschAlgorithm {
    private UndirectedGraph graph;

    @Data
    @AllArgsConstructor
    private static final class VisitJob {
        private final int startVertex;
        private final Set<Integer> candidates;
        private final Set<Integer> excluded;
    }

    private final class VisitProducer {
        private final Set<Integer> excluded = new HashSet<>(graph.order());

        VisitJob createJob(Integer startVtx) {
            var neighbours = graph.neighbours(startVtx);
            assert !neighbours.isEmpty();
            var neighbouringCandidates = util.Difference(neighbours, excluded)
                    .collect(Collectors.toCollection(HashSet::new));
            VisitJob job = null;
            if (neighbouringCandidates.isEmpty()) {
                assert !util.AreDisjoint(neighbours, excluded);
            } else {
                var neighbouringExcluded = util.Intersect(neighbours, excluded)
                        .collect(Collectors.toCollection(HashSet::new));
                job = new VisitJob(startVtx, neighbouringCandidates, neighbouringExcluded);
            }
            excluded.add(startVtx);
            return job;
        }
    }

    private final class Visitor {
        Stream<Collection<Integer>> visit(VisitJob job) {
            var reporter = new SimpleReporter();
            BronKerboschPivot.visit(graph, reporter,
                    PivotChoice.MaxDegree,
                    PivotChoice.MaxDegree,
                    job.candidates,
                    job.excluded,
                    List.of(job.startVertex));
            return reporter.cliques.stream();
        }
    }

    @Override
    public void explore(UndirectedGraph graph, Reporter reporter) {
        this.graph = graph;
        var visitProducer = new VisitProducer();
        var visitor = new Visitor();
        var ordering = new DegeneracyOrdering(graph, -1);
        var cliques = ordering.stream()
                .mapToObj(visitProducer::createJob)
                .filter(Objects::nonNull)
                .collect(Collectors.toList())
                .parallelStream()
                .flatMap(visitor::visit)
                .collect(Collectors.toList());
        ((SimpleReporter) reporter).cliques.addAll(cliques);
    }
}
