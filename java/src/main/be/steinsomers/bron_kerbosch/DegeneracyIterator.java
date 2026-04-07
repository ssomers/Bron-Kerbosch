package be.steinsomers.bron_kerbosch;

import com.sun.jdi.request.InvalidRequestStateException;

import java.util.*;
import java.util.function.Supplier;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

final class DegeneracyIterator implements PrimitiveIterator.OfInt {
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
    private Optional<Integer> previous_pick = Optional.empty();

    DegeneracyIterator(final UndirectedGraph graph) {
        this.graph = graph;
        final var order = graph.order();
        priority_per_vertex = new int[order];
        queue = new SimplePriorityQueue<>(graph.max_degree());
        for (int candidate = 0; candidate < order; ++candidate) {
            final var priority = graph.degree(candidate);
            if (priority > 0) {
                priority_per_vertex[candidate] = priority;
                queue.put(priority, candidate);
                num_left_to_pick += 1;
            }
        }
    }

    @Override
    public boolean hasNext() {
        if (previous_pick.isPresent()) {
            for (final var v : graph.neighbours(previous_pick.get())) {
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
            assert num_left_to_pick >= 0;
            previous_pick = Optional.empty();
        }
        return num_left_to_pick > 0;
    }

    @Override
    public int nextInt() {
        if (previous_pick.isPresent()) {
            throw new InvalidRequestStateException("nextInt before hasNext");
        }
        assert IntStream.range(0, priority_per_vertex.length).allMatch(v -> queue.contains(priority_per_vertex[v], v));
        assert num_left_to_pick > 0;
        while (num_left_to_pick > 0) {
            var pick = queue.pop();
            if (priority_per_vertex[pick] > 0) {
                priority_per_vertex[pick] = 0;
                num_left_to_pick -= 1;
                previous_pick = Optional.of(pick);
                return pick;
            }
        }
        throw new NoSuchElementException("nextInt couldn't pop");
    }

    public boolean isCandidate(int v) {
        return priority_per_vertex[v] > 0;
    }

    private static final class SimplePriorityQueue<T> {
        private final List<ArrayList<T>> stack_per_priority;

        SimplePriorityQueue(final int maxPriority) {
            stack_per_priority = Stream
                    .generate((Supplier<ArrayList<T>>) ArrayList::new)
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
