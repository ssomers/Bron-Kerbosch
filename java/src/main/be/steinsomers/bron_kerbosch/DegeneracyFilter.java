package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.List;
import java.util.NoSuchElementException;
import java.util.PrimitiveIterator;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.function.Supplier;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

final class DegeneracyFilter implements PrimitiveIterator.OfInt {
    private final UndirectedGraph graph;
    // Possible values of priority_per_vertex (after initialization):
    //   0: never queued because not connected (degree 0),
    //      or no longer queued because it has been yielded itself,
    //      or no longer queued because all neighbours have been yielded
    //   1...maxPriority: candidates queued with priority (degree - #of yielded neighbours)
    private final int[] priority_per_vertex;
    @SuppressWarnings("UseOfConcreteClass")
    private final SimplePriorityQueue<Integer> queue;
    private int num_left_to_pick;

    DegeneracyFilter(final UndirectedGraph graph) {
        this.graph = graph;
        final var order = graph.order();
        var maxPriority = 0;
        priority_per_vertex = new int[order];
        for (int candidate = 0; candidate < order; ++candidate) {
            final var degree = graph.degree(candidate);
            if (degree > 0) {
                maxPriority = Math.max(maxPriority, degree);
                priority_per_vertex[candidate] = degree;
                num_left_to_pick += 1;
            }
        }
        queue = new SimplePriorityQueue<>(maxPriority, num_left_to_pick);
        for (int candidate = 0; candidate < order; ++candidate) {
            final var priority = priority_per_vertex[candidate];
            if (priority != 0) {
                queue.put(priority, candidate);
            }
        }
    }

    @Override
    public boolean hasNext() {
        return num_left_to_pick > 0;
    }

    @Override
    public int nextInt() {
        assert hasNext();
        assert IntStream.range(0, priority_per_vertex.length).allMatch(v -> queue.contains(priority_per_vertex[v], v));
        while (num_left_to_pick > 0) {
            var pick = queue.pop();
            if (priority_per_vertex[pick] > 0) {
                priority_per_vertex[pick] = 0;
                num_left_to_pick -= 1;
                for (final var v : graph.neighbours(pick)) {
                    final var oldPriority = priority_per_vertex[v];
                    if (oldPriority != 0) {
                        // Requeue with a more urgent priority or dequeue.
                        // Don't bother to remove the original entry from the queue,
                        // since the vertex will be skipped when popped, and thanks to
                        // num_left_to_pick we might not need to pop it at all.
                        final var newPriority = oldPriority - 1;
                        priority_per_vertex[v] = newPriority;
                        if (newPriority != 0) {
                            queue.put(newPriority, v);
                        } else {
                            num_left_to_pick -= 1;
                        }
                    }
                }
                return pick;
            }
        }
        throw new NoSuchElementException();
    }

    private static final class SimplePriorityQueue<T> {
        private final List<ArrayList<T>> stack_per_priority;

        SimplePriorityQueue(final int maxPriority, final int sizeHint) {
            stack_per_priority = Stream
                    .generate((Supplier<ArrayList<T>>) () -> new ArrayList<>(sizeHint))
                    .limit(maxPriority)
                    .collect(Collectors.toCollection(ArrayList::new));
        }

        void put(final int priority, final T elt) {
            assert priority > 0;
            final var stack = stack_per_priority.get(priority - 1);
            stack.add(elt);
        }

        T pop() {
            for (final var stack : stack_per_priority) {
                if (!stack.isEmpty()) {
                    final var last = stack.size() - 1;
                    final var elt = stack.get(last);
                    stack.remove(last);
                    return elt;
                }
            }
            throw new NoSuchElementException("attempt to pop more than was put");
        }

        // Inefficiently check that the queue contains the element at the right priority, if any
        boolean contains(final int priority, final T elt) {
            return priority == 0 || stack_per_priority.get(priority - 1).contains(elt);
        }
    }

    IntStream stream() {
        final var characteristics = Spliterator.ORDERED
                | Spliterator.DISTINCT
                | Spliterator.NONNULL
                | Spliterator.IMMUTABLE;
        final var spliterator = Spliterators.spliteratorUnknownSize(this, characteristics);
        return StreamSupport.intStream(spliterator, false);
    }
}
