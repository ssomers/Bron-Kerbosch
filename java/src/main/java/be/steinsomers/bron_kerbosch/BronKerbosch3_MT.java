package be.steinsomers.bron_kerbosch;

import lombok.Data;
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
    private static final int NUM_VISITING_THREADS = 5;
    private static final int CLEAN_END_VERTEX = -1;
    private static final int DIRTY_END_VERTEX = -2;
    private UndirectedGraph graph;
    private BlockingQueue<VisitJob> startQueue;
    private BlockingQueue<VisitJob> visitQueue;
    private Collection<int[]> cliques;

    @Data
    @RequiredArgsConstructor
    private static final class VisitJob {
        private final int startVertex;
        private Set<Integer> mut_candidates;
        private Set<Integer> mut_excluded;
    }

    @NoArgsConstructor
    private final class StartProducer extends Thread {
        @Override
        public void run() {
            try {
                Iterable<Integer> vertices = () -> new DegeneracyOrdering(graph, -1);
                for (var v : vertices) {
                    startQueue.put(new VisitJob(v));
                }
                startQueue.put(new VisitJob(CLEAN_END_VERTEX));
            } catch (InterruptedException consumed) {
                startQueue.clear();
                startQueue.add(new VisitJob(DIRTY_END_VERTEX));
            }
        }
    }

    @NoArgsConstructor
    private final class VisitProducer extends Thread {
        @Override
        public void run() {
            try {
                Set<Integer> mut_excluded = new HashSet<>(graph.order());
                VisitJob job;
                while ((job = startQueue.take()).startVertex >= 0) {
                    var v = job.startVertex;
                    var neighbours = graph.neighbours(v);
                    assert !neighbours.isEmpty();
                    job.mut_candidates = util.Difference(neighbours, mut_excluded)
                            .collect(Collectors.toCollection(HashSet::new));
                    if (job.mut_candidates.isEmpty()) {
                        assert !util.AreDisjoint(neighbours, mut_excluded);
                    } else {
                        job.mut_excluded = util.Intersect(neighbours, mut_excluded)
                                .collect(Collectors.toCollection(HashSet::new));
                        visitQueue.put(job);
                    }
                    mut_excluded.add(v);
                }
                for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
                    visitQueue.put(job);
                }
            } catch (InterruptedException consumed) {
                visitQueue.clear();
                for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
                    visitQueue.add(new VisitJob(DIRTY_END_VERTEX));
                }
            }
        }
    }

    @NoArgsConstructor
    private final class Visitor extends Thread {
        @Override
        public void run() {
            try {
                VisitJob job;
                while ((job = visitQueue.take()).startVertex >= 0) {
                    BronKerboschPivot.visit(graph, clique -> cliques.add(clique),
                            PivotChoice.MaxDegreeLocal,
                            job.mut_candidates,
                            job.mut_excluded,
                            new int[]{job.startVertex});
                }
                if (job.startVertex == DIRTY_END_VERTEX) {
                    cliques.clear();
                }
            } catch (InterruptedException consumed) {
                cliques.clear();
            }
        }
    }

    @Override
    public Stream<int[]> explore(UndirectedGraph graph) throws InterruptedException {
        this.graph = graph;
        cliques = Collections.synchronizedCollection(new ArrayDeque<>());
        startQueue = new ArrayBlockingQueue<>(64);
        visitQueue = new ArrayBlockingQueue<>(64);
        new StartProducer().start();
        new VisitProducer().start();
        var visitors = new Visitor[NUM_VISITING_THREADS];
        for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
            visitors[i] = new Visitor();
            visitors[i].start();
        }
        for (int i = 0; i < NUM_VISITING_THREADS; ++i) {
            visitors[i].join();
        }
        return cliques.stream();
    }
}
