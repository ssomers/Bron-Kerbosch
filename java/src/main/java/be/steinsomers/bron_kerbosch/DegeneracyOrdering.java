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

final class DegeneracyOrdering implements PrimitiveIterator.OfInt {
    private final UndirectedGraph graph;
    // priority_per_vertex:
    // If priority is 0, vertex was already picked or was always irrelevant (unconnected);
    // otherwise, vertex is still queued and priority = degree + 1 - number of picked neighbours.
    private final int[] priority_per_vertex;
    @SuppressWarnings("UseOfConcreteClass")
    private final SimplePriorityQueue<Integer> queue;
    private int num_left_to_pick;

    DegeneracyOrdering(UndirectedGraph graph, int drop) {
        assert drop <= 0;
        this.graph = graph;
        var order = graph.order();
        var maxPriority = 0;
        priority_per_vertex = new int[order];
        var numCandidates = 0;
        for (int candidate = 0; candidate < order; ++candidate) {
            var degree = graph.degree(candidate);
            if (degree > 0) {
                var priority = degree + 1;
                maxPriority = Math.max(maxPriority, priority);
                priority_per_vertex[candidate] = priority;
                numCandidates += 1;
            }
        }
        queue = new SimplePriorityQueue<>(maxPriority, numCandidates);
        for (int candidate = 0; candidate < order; ++candidate) {
            var priority = priority_per_vertex[candidate];
            if (priority != 0) {
                queue.put(priority, candidate);
            }
        }
        num_left_to_pick = numCandidates + drop;
    }

    @Override
    public boolean hasNext() {
        return num_left_to_pick > 0;
    }

    @Override
    public int nextInt() {
        assert IntStream.range(0, priority_per_vertex.length).allMatch(v -> queue.ensure(priority_per_vertex[v], v));
        var i = queue.pop();
        while (priority_per_vertex[i] == 0) {
            // v was requeued with a more urgent priority and therefore already picked
            i = queue.pop();
        }

        priority_per_vertex[i] = 0;
        for (var v : graph.neighbours(i)) {
            var oldPriority = priority_per_vertex[v];
            if (oldPriority != 0) {
                // Since this is an unvisited neighbour of a vertex just being picked,
                // its priority can't be down to the minimum.
                var newPriority = oldPriority - 1;
                assert newPriority > 0;
                // Requeue with a more urgent priority, but don't bother to remove
                // the original entry - it will be skipped if it's reached at all.
                priority_per_vertex[v] = newPriority;
                queue.put(newPriority, v);
            }
        }
        num_left_to_pick -= 1;
        return i;
    }

    private static final class SimplePriorityQueue<T> {
        private final List<ArrayList<T>> stack_per_priority;

        SimplePriorityQueue(int maxPriority, int sizeHint) {
            stack_per_priority = Stream
                    .generate((Supplier<ArrayList<T>>) () -> new ArrayList<>(sizeHint))
                    .limit(maxPriority)
                    .collect(Collectors.toCollection(ArrayList::new));
        }

        void put(int priority, T elt) {
            var stack = stack_per_priority.get(priority - 1);
            stack.add(elt);
        }

        T pop() {
            for (var stack : stack_per_priority) {
                if (!stack.isEmpty()) {
                    var last = stack.size() - 1;
                    var elt = stack.get(last);
                    stack.remove(last);
                    return elt;
                }
            }
            throw new NoSuchElementException("attempt to pop more than was put");
        }

        // Inefficiently check that the queue contains the element at the right priority, if any
        boolean ensure(int priority, T elt) {
            return priority == 0 || stack_per_priority.get(priority - 1).contains(elt);
        }
    }

    IntStream stream() {
        var characteristics = Spliterator.ORDERED
                | Spliterator.DISTINCT
                | Spliterator.NONNULL
                | Spliterator.IMMUTABLE;
        var spliterator = Spliterators.spliterator(this, num_left_to_pick, characteristics);
        return StreamSupport.intStream(spliterator, false);
    }
}
