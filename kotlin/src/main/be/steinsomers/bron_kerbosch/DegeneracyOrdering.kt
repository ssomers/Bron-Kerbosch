package be.steinsomers.bron_kerbosch

import java.util.PrimitiveIterator
import java.util.Spliterator
import java.util.Spliterators
import java.util.stream.IntStream
import java.util.stream.StreamSupport
import kotlin.math.max

internal class DegeneracyOrdering(private val graph: UndirectedGraph, drop: Int) : PrimitiveIterator.OfInt {
    // priority_per_vertex:
    // If priority is 0, vertex was already picked or was always irrelevant (unconnected);
    // otherwise, vertex is still queued and priority = degree + 1 - number of picked neighbours.
    private val priorityPerVertex: IntArray
    private val queue: SimplePriorityQueue<Int>
    private var numLeftToPick: Int

    init {
        require(drop >= 0)
        var maxPriority = 0
        priorityPerVertex = IntArray(graph.order)
        var numCandidates = 0
        for (candidate in 0..<graph.order) {
            val degree = graph.degree(candidate)
            if (degree > 0) {
                val priority = degree + 1
                maxPriority = max(maxPriority, priority)
                priorityPerVertex[candidate] = priority
                numCandidates += 1
            }
        }
        queue = SimplePriorityQueue(maxPriority, numCandidates)
        for (candidate in 0..<graph.order) {
            val priority = priorityPerVertex[candidate]
            if (priority != 0) {
                queue.put(priority, candidate)
            }
        }
        numLeftToPick = numCandidates - drop
    }

    override fun hasNext(): Boolean {
        return numLeftToPick > 0
    }

    override fun nextInt(): Int {
        Debug.assert { priorityPerVertex.indices.all { v -> queue.ensure(priorityPerVertex[v], v) } }
        var pick = queue.pop()
        while (priorityPerVertex[pick] == 0) {
            // v was requeued with a more urgent priority and therefore already picked
            pick = queue.pop()
        }

        priorityPerVertex[pick] = 0
        for (v in graph.neighbours(pick)) {
            val oldPriority = priorityPerVertex[v]
            if (oldPriority != 0) {
                // Since this is an unvisited neighbour of a vertex just being picked,
                // its priority can't be down to the minimum.
                val newPriority = oldPriority - 1
                require(newPriority > 0)
                // Requeue with a more urgent priority, but don't bother to remove
                // the original entry - it will be skipped if it's reached at all.
                priorityPerVertex[v] = newPriority
                queue.put(newPriority, v)
            }
        }
        numLeftToPick -= 1
        return pick
    }

    private class SimplePriorityQueue<T>(maxPriority: Int, private val sizeHint: Int) {
        private val stackPerPriority = Array<ArrayList<T>>(size = maxPriority) { _ -> ArrayList(sizeHint) }

        fun put(priority: Int, elt: T) {
            stackPerPriority[priority - 1].add(elt)
        }

        fun pop(): T {
            for (stack in stackPerPriority) {
                if (stack.isNotEmpty()) {
                    val last = stack.size - 1
                    val elt = stack[last]
                    stack.removeAt(last)
                    return elt
                }
            }
            throw NoSuchElementException("attempt to pop more than was put")
        }

        // Inefficiently check that the queue contains the element at the right priority, if any
        fun ensure(priority: Int, elt: T): Boolean {
            return priority == 0 || stackPerPriority[priority - 1].contains(elt)
        }
    }

    fun stream(): IntStream {
        val characteristics = (Spliterator.ORDERED
                or Spliterator.DISTINCT
                or Spliterator.NONNULL
                or Spliterator.IMMUTABLE)
        val spliterator = Spliterators.spliterator(this, numLeftToPick.toLong(), characteristics)
        return StreamSupport.intStream(spliterator, false)
    }

    override fun remove() {
        throw NotImplementedError("I'm a read-only iterator")
    }
}
