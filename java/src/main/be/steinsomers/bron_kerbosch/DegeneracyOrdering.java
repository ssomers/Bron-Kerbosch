package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.OptionalInt;
import java.util.stream.IntStream;

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
        var max_priority = 0;
        priority_per_vertex = new int[order];
        var num_candidates = 0;
        for (int c = 0; c < order; ++c) {
            var degree = graph.degree(c);
            if (degree > 0) {
                var priority = degree + 1;
                max_priority = Math.max(max_priority, priority);
                priority_per_vertex[c] = priority;
                num_candidates += 1;
            }
        }
        queue = new SimplePriorityQueue(max_priority);
        for (int c = 0; c < order; ++c) {
            var priority = priority_per_vertex[c];
            if (priority != 0) {
                queue.put(priority, c);
            }
        }
        num_left_to_pick = num_candidates + drop;
    }

    @Override
    public boolean hasNext() {
        return num_left_to_pick > 0;
    }

    @Override
    public Integer next() {
        assert IntStream.range(0, priority_per_vertex.length).allMatch(v -> priority_per_vertex[v] == 0 || queue.contains(priority_per_vertex[v], v));
        var i = queue.pop().orElseThrow();
        while (priority_per_vertex[i] == 0) {
            // v was requeued with a more urgent priority and therefore already picked
            i = queue.pop().orElseThrow();
        }

        priority_per_vertex[i] = 0;
        for (var v : graph.neighbours(i)) {
            var old_priority = priority_per_vertex[v];
            if (old_priority != 0) {
                // Since this is an unvisited neighbour of a vertex just being picked,
                // its priority can't be down to the minimum.
                var new_priority = old_priority - 1;
                assert new_priority > 0;
                // Requeue with a more urgent priority, but don't bother to remove
                // the original entry - it will be skipped if it's reached at all.
                priority_per_vertex[v] = new_priority;
                queue.put(new_priority, v);
            }
        }
        num_left_to_pick -= 1;
        return i;
    }

    static final class SimplePriorityQueue {
        private final ArrayList<ArrayList<Integer>> stack_per_priority;

        SimplePriorityQueue(int max_priority) {
            stack_per_priority = new ArrayList<>(max_priority);
            IntStream.range(0, max_priority).forEach(i -> stack_per_priority.add(new ArrayList<>()));
        }

        void put(int priority, int elt) {
            var stack = stack_per_priority.get(priority-1);
            if (stack == null) {
                stack = new ArrayList<>();
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
