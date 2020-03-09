package be.steinsomers.bron_kerbosch;

import lombok.AllArgsConstructor;
import lombok.NonNull;

import java.util.Spliterator;
import java.util.function.Consumer;

public final class BronKerboschSpliterator implements Spliterator<int[]> {
    @FunctionalInterface
    interface Generator {
        boolean findNextVertex(VtxConsumer consumer);
    }

    interface VtxConsumer {
        void acceptClique(int vertex);

        void diveDeeper(int vertex, Generator subGenerator);
    }

    @AllArgsConstructor
    private static final class GeneratorLevel {
        public final int vertex;
        public final @NonNull Generator generator;
        public final int numVertices;
        public final GeneratorLevel below;
    }

    private GeneratorLevel top;

    BronKerboschSpliterator(int startVtx, @NonNull Generator startGen) {
        top = new GeneratorLevel(startVtx, startGen, startVtx >= 0 ? 1 : 0, null);
    }

    public int characteristics() {
        return DISTINCT | NONNULL | IMMUTABLE;
    }

    public long estimateSize() {
        return Long.MAX_VALUE;
    }

    @SuppressWarnings("ReturnOfNull")
    public Spliterator<int[]> trySplit() {
        return null;
    }

    private int[] collectClique(int vertex) {
        var clique = new int[top.numVertices + 1];
        int i = 0;
        clique[i++] = vertex;
        var level = top;
        while (true) {
            assert level.vertex >= 0;
            clique[i++] = level.vertex;
            if (level.numVertices == 1) {
                assert i == top.numVertices + 1;
                return clique;
            }
            level = level.below;
        }
    }

    public boolean tryAdvance(Consumer<? super int[]> consumer) {
        while (top != null) {
            var curLevel = top;
            var generator = curLevel.generator;
            if (generator.findNextVertex(new VtxConsumer() {
                @Override
                public void acceptClique(int vertex) {
                    consumer.accept(collectClique(vertex));
                }

                @Override
                public void diveDeeper(int vertex, Generator subGenerator) {
                    top = new GeneratorLevel(vertex, subGenerator, top.numVertices + 1, top);
                }
            })) {
                if (top == curLevel) {
                    return true;
                }
            } else {
                top = top.below;
            }
        }
        return false;
    }
}
