package be.steinsomers.bron_kerbosch;

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

public final class BronKerbosch3_ST implements BronKerboschAlgorithm {
    @Override
    public void explore(final UndirectedGraph graph, final CliqueConsumer cliqueConsumer) {
        final var worker = new Worker(graph, cliqueConsumer);
        worker.work();
    }

    private record VisitJob(int startVertex, Set<Integer> candidates, Set<Integer> excluded) {
    }

    private record Worker(UndirectedGraph graph, CliqueConsumer cliqueConsumer) {
        void work() {
            final var visitProducer = new VisitProducer();
            final var visitor = new Visitor();
            final var ordering = new DegeneracyIterator(graph);
            ordering.stream()
                    .mapToObj(visitProducer::createJob)
                    .toList()
                    .parallelStream()
                    .forEach(visitor::visit);
        }

        private final class VisitProducer {
            private final boolean[] excluded = new boolean[graph.order()];

            VisitJob createJob(final Integer startVtx) {
                final var neighbours = graph.neighbours(startVtx);
                final var neighbouringCandidates = neighbours.stream().filter(v -> !excluded[v])
                        .collect(Collectors.toCollection(HashSet::new));
                final var neighbouringExcluded = neighbours.stream().filter(v -> excluded[v])
                        .collect(Collectors.toCollection(HashSet::new));
                assert !neighbouringCandidates.isEmpty();
                excluded[startVtx] = true;
                return new VisitJob(startVtx, neighbouringCandidates, neighbouringExcluded);
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
