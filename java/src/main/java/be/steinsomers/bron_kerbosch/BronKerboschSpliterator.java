package be.steinsomers.bron_kerbosch;

import lombok.AllArgsConstructor;
import lombok.NonNull;

import java.util.ArrayDeque;
import java.util.Spliterator;
import java.util.function.Consumer;

final class BronKerboschSpliterator implements Spliterator<int[]> {
    private final ArrayDeque<GeneratorLevel> queue;

    @FunctionalInterface
    interface Generator {
        // Find the next vertex that completes a maximal clique, if any.
        // The generator feeds it to cliqueConsumer and immediately return true.
        // If cliqueConsumer is null, it's logically impossible to find a clique.
        boolean findNextVertex(Consumer<Integer> cliqueConsumer, VertexVisitQueue recursionQueue);
    }

    @FunctionalInterface
    interface VertexVisitQueue {
        void offer(int vertex, Generator subGenerator);
    }

    @FunctionalInterface
    interface CliqueVisitQueue {
        void offer(int[] vertex, Generator subGenerator);
    }

    @AllArgsConstructor
    private static final class GeneratorLevel {
        public final @NonNull Generator generator;
        public final int[] cliqueInProgress;

        private int[] collectClique(int vertex) {
            return util.Append(cliqueInProgress, vertex);
        }

        public boolean tryLevel(Consumer<? super int[]> consumer,
                                CliqueVisitQueue recursionQueue) {
            return generator.findNextVertex(
                    vertex -> consumer.accept(collectClique(vertex)),
                    (vertex, subGen) -> recursionQueue.offer(collectClique(vertex), subGen)
            );
        }
    }

    BronKerboschSpliterator(int size) {
        queue = new ArrayDeque<>(size);
    }

    BronKerboschSpliterator(@NonNull Generator startGen) {
        queue = new ArrayDeque<>();
        boolean phantomFound = startGen.findNextVertex(null, this::offerInitial);
        assert !phantomFound;
    }

    public int characteristics() {
        return DISTINCT | NONNULL | IMMUTABLE;
    }

    public long estimateSize() {
        return Long.MAX_VALUE;
    }

    @SuppressWarnings("ReturnOfNull")
    public Spliterator<int[]> trySplit() {
        var halfSize = queue.size() / 2;
        if (halfSize == 0) {
            return null;
        } else {
            var half = new BronKerboschSpliterator(halfSize);
            for (int i = 0; i < halfSize; ++i) {
                half.queue.offer(queue.pop());
            }
            return half;
        }
    }

    public boolean tryAdvance(Consumer<? super int[]> consumer) {
        while (!queue.isEmpty()) {
            var level = queue.peek();
            if (level.tryLevel(consumer, this::offer)) {
                return true;
            }
            queue.pop();
        }
        return false;
    }

    public void offerInitial(int vertex, Generator subGenerator) {
        queue.offer(new GeneratorLevel(subGenerator, new int[]{vertex}));
    }

    private void offer(int[] cliqueInProgress, Generator subGenerator) {
        queue.offer(new GeneratorLevel(subGenerator, cliqueInProgress));
    }
}
