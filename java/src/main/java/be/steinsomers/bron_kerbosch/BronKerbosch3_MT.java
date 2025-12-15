package be.steinsomers.bron_kerbosch;

import lombok.NoArgsConstructor;
import lombok.RequiredArgsConstructor;

import java.util.ArrayDeque;
import java.util.Collection;
import java.util.Collections;
import java.util.HashSet;
import java.util.Set;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public final class BronKerbosch3_MT implements BronKerboschAlgorithm {
    @Override
    public Stream<int[]> explore(UndirectedGraph graph) throws InterruptedException {
        var worker = new Worker(graph);
        return worker.stream();
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

        final boolean ok() {
            return startVertex >= 0;
        }
    }

    private static final class VisitJob extends StartJob {
        VisitJob(StartJob start, Set<Integer> candidates, Set<Integer> excluded) {
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
        private final BlockingQueue<StartJob> startQueue;
        private final BlockingQueue<VisitJob> visitQueue;
        private final Collection<int[]> cliques;
        private final StartProducer startProducer = new StartProducer();
        private final VisitProducer visitProducer = new VisitProducer();

        @NoArgsConstructor
        private final class StartProducer implements Runnable {
            @Override
            public void run() {
                try {
                    Iterable<Integer> vertices = () -> new DegeneracyOrdering(graph, -1);
                    for (var v : vertices) {
                        startQueue.put(new StartJob(v));
                    }
                    startQueue.put(StartJob.CleanEnd());
                } catch (InterruptedException consumed) {
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
                    Set<Integer> mut_excluded = new HashSet<>(graph.order());
                    StartJob startJob;
                    while ((startJob = startQueue.take()).ok()) {
                        var v = startJob.startVertex;
                        var neighbours = graph.neighbours(v);
                        assert !neighbours.isEmpty();
                        var candidates = util.Difference(neighbours, mut_excluded)
                                .collect(Collectors.toCollection(HashSet::new));
                        if (candidates.isEmpty()) {
                            assert !util.AreDisjoint(neighbours, mut_excluded);
                        } else {
                            var excluded = util.Intersect(neighbours, mut_excluded)
                                    .collect(Collectors.toCollection(HashSet::new));
                            VisitJob job = new VisitJob(startJob, candidates, excluded);
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
                    while ((job = visitQueue.take()).startVertex >= 0) {
                        BronKerboschPivot.visit(graph, cliques::add,
                                PivotChoice.MaxDegreeLocal,
                                job.candidates,
                                job.excluded,
                                new int[]{job.startVertex});
                    }
                } catch (InterruptedException consumed) {
                    cliques.clear();
                }
            }
        }

        private Worker(UndirectedGraph graph) {
            this.graph = graph;
            @SuppressWarnings("NumericCastThatLosesPrecision")
            var initialCap = (int) Math.floor(Math.sqrt(graph.size()));
            cliques = Collections.synchronizedCollection(new ArrayDeque<>(initialCap));
            startQueue = new ArrayBlockingQueue<>(64);
            visitQueue = new ArrayBlockingQueue<>(64);
        }

        public Stream<int[]> stream() throws InterruptedException {
            new Thread(startProducer).start();
            new Thread(visitProducer).start();
            var visitors = new Thread[NUM_VISITING_THREADS];
            for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
                visitors[i] = new Thread(new Visitor());
                visitors[i].start();
            }
            for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
                visitors[i].join();
            }
            return cliques.stream();
        }
    }
}
