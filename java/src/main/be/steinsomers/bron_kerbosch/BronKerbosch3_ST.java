package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.Objects;
import java.util.Set;
import java.util.function.Consumer;
import java.util.stream.Collectors;

public final class BronKerbosch3_ST implements BronKerboschAlgorithm {
    @Override
    public void explore(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer) {
        final var worker = new Worker(graph, cliqueConsumer);
        worker.work();
    }

    private record VisitJob(int startVertex, Set<Integer> candidates, Set<Integer> excluded) {
    }

    private record Worker(UndirectedGraph graph, Consumer<int[]> cliqueConsumer) {
            void work() {
                final var visitProducer = new VisitProducer();
                final var visitor = new Visitor();
                final var ordering = new DegeneracyFilter(graph);
                ordering.stream()
                        .mapToObj(visitProducer::createJob)
                        .filter(Objects::nonNull)
                        .toList()
                        .parallelStream()
                        .forEach(visitor::visit);
            }

            private final class VisitProducer {
                private final Set<Integer> excluded = new HashSet<>(graph.order());

                VisitJob createJob(final Integer startVtx) {
                    final var neighbours = graph.neighbours(startVtx);
                    assert !neighbours.isEmpty();
                    final var neighbouringCandidates = util.Difference(neighbours, excluded)
                            .collect(Collectors.toCollection(HashSet::new));
                    VisitJob job = null;
                    if (neighbouringCandidates.isEmpty()) {
                        assert !util.AreDisjoint(neighbours, excluded);
                    } else {
                        final var neighbouringExcluded = util.Intersect(neighbours, excluded)
                                .collect(Collectors.toCollection(HashSet::new));
                        job = new VisitJob(startVtx, neighbouringCandidates, neighbouringExcluded);
                    }
                    excluded.add(startVtx);
                    return job;
                }
            }

            private final class Visitor {
                void visit(final VisitJob job) {
                    BronKerboschPivot.visit(graph, cliqueConsumer,
                            PivotChoice.MaxDegreeLocal,
                            job.candidates,
                            job.excluded,
                            new int[]{job.startVertex});
                }
            }
        }
}
