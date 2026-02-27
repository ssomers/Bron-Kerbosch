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

    DegeneracyFilter(final UndirectedGraph graph) {
        this.graph = graph;
        final var order = graph.order();
        queue = new SimplePriorityQueue<>(graph.max_degree());
        priority_per_vertex = new int[order];
        for (int v = 0; v < order; ++v) {
            final var priority = graph.degree(v);
            priority_per_vertex[v] = priority;
            queue.insert(v, priority);
        }
    }

    @Override
    public boolean hasNext() {
        return !queue.empty();
    }

    @Override
    public int nextInt() {
        assert hasNext();
        assert IntStream.range(0, priority_per_vertex.length).allMatch(v -> queue.contains(priority_per_vertex[v], v));
        while (hasNext()) {
            var pick = queue.pop();
            if (priority_per_vertex[pick] > 0) {
                priority_per_vertex[pick] = 0;
                queue.forget(pick);
                for (final var v : graph.neighbours(pick)) {
                    final var oldPriority = priority_per_vertex[v];
                    if (oldPriority != 0) {
                        final var newPriority = oldPriority - 1;
                        priority_per_vertex[v] = newPriority;
                        queue.promote(v, newPriority);
                    }
                }
                return pick;
            }
        }
        throw new NoSuchElementException();
    }

    private static final class SimplePriorityQueue<T> {
        private final List<ArrayList<T>> stack_per_priority;
        private int num_left_to_pick;

        SimplePriorityQueue(final int maxPriority) {
            stack_per_priority = Stream
                    .generate((Supplier<ArrayList<T>>) ArrayList::new)
                    .limit(maxPriority)
                    .collect(Collectors.toCollection(ArrayList::new));
            num_left_to_pick = 0;
        }

        boolean empty() {
            return num_left_to_pick == 0;
        }

        void insert(final T elt, final int priority) {
            if (priority > 0) {
                stack_per_priority.get(priority - 1).add(elt);
                num_left_to_pick += 1;
            }
        }

        // Requeue with a more urgent priority or dequeue.
        // Don't bother to remove the original entry from the queue,
        // since the vertex will be skipped when popped, and thanks to
        // num_left_to_pick we might not need to pop it at all.
        void promote(final T elt, final int priority) {
            if (priority > 0) {
                stack_per_priority.get(priority - 1).add(elt);
            } else {
                forget(elt);
            }
        }

        void forget(final T ignoredElt) {
            assert num_left_to_pick > 0;
            num_left_to_pick -= 1;
        }

        // We may return an element already popped, even though it was passed to forget,
        // in case its priority was promoted earlier on. That's why we do not count 
        // the element as picked, but wait for the caller to forget it. The caller must
        // somehow ensure to forget the same element only once.
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
