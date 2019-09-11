package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.NoSuchElementException;
import java.util.OptionalInt;
import java.util.function.Supplier;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

final class DegeneracyOrdering implements Iterator<Integer> {
    private final UndirectedGraph graph;
    // priority_per_vertex:
    // If priority is 0, vertex was already picked or was always irrelevant (unconnected);
    // otherwise, vertex is still queued and priority = degree + 1 - number of picked neighbours.
    private final int[] priority_per_vertex;
    private final SimplePriorityQueue queue;
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
        queue = new SimplePriorityQueue(maxPriority, numCandidates);
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
    public Integer next() {
        assert IntStream.range(0, priority_per_vertex.length)
                .allMatch(v -> priority_per_vertex[v] == 0
                        || queue.contains(priority_per_vertex[v], v));
        var i = queue.pop().orElseThrow(() -> new NoSuchElementException("gotcha"));
        while (priority_per_vertex[i] == 0) {
            // v was requeued with a more urgent priority and therefore already picked
            i = queue.pop().orElseThrow();
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

    private static final class SimplePriorityQueue {
        private final List<ArrayList<Integer>> stack_per_priority;
        private final int sizeHint;

        SimplePriorityQueue(int maxPriority, int sizeHint) {
            stack_per_priority = Stream
                    .generate((Supplier<ArrayList<Integer>>) ArrayList::new)
                    .limit(maxPriority)
                    .collect(Collectors.toCollection(ArrayList::new));
            this.sizeHint = sizeHint;
        }

        void put(int priority, int elt) {
            var stack = stack_per_priority.get(priority - 1);
            if (stack == null) {
                stack = new ArrayList<>(sizeHint);
            }
            stack.add(elt);
        }

        OptionalInt pop() {
            for (var stack : stack_per_priority) {
                if (!stack.isEmpty()) {
                    var last = stack.size() - 1;
                    var elt = stack.get(last);
                    stack.remove(last);
                    return OptionalInt.of(elt);
                }
            }
            return OptionalInt.empty();
        }

        boolean contains(int priority, int elt) {
            return stack_per_priority.get(priority - 1).contains(elt);
        }
    }
}
