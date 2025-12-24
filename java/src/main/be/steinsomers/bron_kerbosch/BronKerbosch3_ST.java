package be.steinsomers.bron_kerbosch;

import lombok.RequiredArgsConstructor;

import java.util.HashSet;
import java.util.Objects;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public final class BronKerbosch3_ST implements BronKerboschAlgorithm {

    @Override
    public Stream<int[]> explore(UndirectedGraph graph) {
        var worker = new Worker(graph);
        return worker.stream();
    }

    private record VisitJob(int startVertex, Set<Integer> candidates, Set<Integer> excluded) {}

    @RequiredArgsConstructor
    private static final class Worker {
        private final UndirectedGraph graph;

            public Stream<int[]> stream() {
                var visitProducer = new VisitProducer();
                var visitor = new Visitor();
                var ordering = new DegeneracyOrdering(graph, -1);
                return ordering.stream()
                        .mapToObj(visitProducer::createJob)
                        .filter(Objects::nonNull)
                        .toList()
                        .parallelStream()
                        .flatMap(visitor::visit);
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
                Stream<int[]> visit(VisitJob job) {
                    Stream.Builder<int[]> cliqueStream = Stream.builder();
                    BronKerboschPivot.visit(graph, cliqueStream,
                            PivotChoice.MaxDegreeLocal,
                            job.candidates,
                            job.excluded,
                            new int[]{job.startVertex});
                    return cliqueStream.build();
                }
            }
        }
}
