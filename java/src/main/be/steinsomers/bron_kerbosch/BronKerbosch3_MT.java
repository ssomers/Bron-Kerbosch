package be.steinsomers.bron_kerbosch;

import lombok.NoArgsConstructor;
import lombok.RequiredArgsConstructor;

import java.util.HashSet;
import java.util.Set;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.function.Consumer;
import java.util.stream.Collectors;

public final class BronKerbosch3_MT implements BronKerboschAlgorithm {
    @Override
    public void explore(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer) throws InterruptedException {
        final var worker = new Worker(graph, cliqueConsumer);
        worker.work();
    }

    @RequiredArgsConstructor
    private static class StartJob {
        private static final int CLEAN_END_VERTEX = -1;
        private static final int DIRTY_END_VERTEX = -2;
        final int startVertex;

        static StartJob CleanEnd() {
            return new StartJob(CLEAN_END_VERTEX);
        }

        static StartJob DirtyEnd() {
            return new StartJob(DIRTY_END_VERTEX);
        }

        final boolean isGenuine() {
            return startVertex >= 0;
        }
    }

    private static final class VisitJob extends StartJob {
        VisitJob(final StartJob start, final Set<Integer> candidates, final Set<Integer> excluded) {
            super(start.startVertex);
            this.candidates = candidates;
            this.excluded = excluded;
        }

        final Set<Integer> candidates;
        final Set<Integer> excluded;
    }

    private static final class Worker {
        private static final int NUM_VISITING_THREADS = 5;

        private final UndirectedGraph graph;
        private final Consumer<int[]> cliqueConsumer;
        private final BlockingQueue<StartJob> startQueue;
        private final BlockingQueue<VisitJob> visitQueue;
        private final StartProducer startProducer = new StartProducer();
        private final VisitProducer visitProducer = new VisitProducer();

        @NoArgsConstructor
        private final class StartProducer implements Runnable {
            @Override
            public void run() {
                try {
                    final Iterable<Integer> vertices = () -> new DegeneracyOrdering(graph, -1);
                    for (final var v : vertices) {
                        startQueue.put(new StartJob(v));
                    }
                    startQueue.put(StartJob.CleanEnd());
                } catch (final InterruptedException consumed) {
                    startQueue.clear();
                    startQueue.add(StartJob.DirtyEnd());
                }
            }
        }

        @NoArgsConstructor
        private final class VisitProducer implements Runnable {
            @SuppressWarnings("UseOfConcreteClass")
            @Override
            public void run() {
                try {
                    final Set<Integer> mut_excluded = new HashSet<>(graph.order());
                    StartJob startJob;
                    while ((startJob = startQueue.take()).isGenuine()) {
                        final var v = startJob.startVertex;
                        final var neighbours = graph.neighbours(v);
                        assert !neighbours.isEmpty();
                        final var candidates = util.Difference(neighbours, mut_excluded)
                                .collect(Collectors.toCollection(HashSet::new));
                        if (candidates.isEmpty()) {
                            assert !util.AreDisjoint(neighbours, mut_excluded);
                        } else {
                            final var excluded = util.Intersect(neighbours, mut_excluded)
                                    .collect(Collectors.toCollection(HashSet::new));
                            final VisitJob job = new VisitJob(startJob, candidates, excluded);
                            visitQueue.put(job);
                        }
                        mut_excluded.add(v);
                    }
                    for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
                        visitQueue.put(new VisitJob(startJob, null, null));
                    }
                } catch (InterruptedException _) {
                    visitQueue.clear();
                    for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
                        visitQueue.add(new VisitJob(StartJob.DirtyEnd(), null, null));
                    }
                }
            }
        }

        @NoArgsConstructor
        private final class Visitor implements Runnable {
            @SuppressWarnings("UseOfConcreteClass")
            @Override
            public void run() {
                try {
                    VisitJob job;
                    while ((job = visitQueue.take()).isGenuine()) {
                        BronKerboschPivot.visit(graph, cliqueConsumer,
                                PivotChoice.MaxDegreeLocal,
                                job.candidates,
                                job.excluded,
                                new int[]{job.startVertex});
                    }
                } catch (final InterruptedException consumed) {
                    // assume that thread remembers it
                }
            }
        }

        private Worker(final UndirectedGraph graph, final Consumer<int[]> cliqueConsumer) {
            this.graph = graph;
            this.cliqueConsumer = cliqueConsumer;
            startQueue = new ArrayBlockingQueue<>(64);
            visitQueue = new ArrayBlockingQueue<>(64);
        }

        void work() throws InterruptedException {
            new Thread(startProducer).start();
            new Thread(visitProducer).start();
            final var visitors = new Thread[NUM_VISITING_THREADS];
            for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
                visitors[i] = new Thread(new Visitor());
                visitors[i].start();
            }
            for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
                visitors[i].join();
            }
            for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
                if (visitors[i].isInterrupted()) {
                    throw new InterruptedException("Some thread got interrupted");
                }
            }
        }
    }
}
