package be.steinsomers.bron_kerbosch

import java.util.*
import java.util.stream.IntStream
import java.util.stream.StreamSupport
import kotlin.math.max

internal class DegeneracyFilter(private val graph: UndirectedGraph) : PrimitiveIterator.OfInt {
    // Possible values of priorityPerVertex (after initialization):
    //   0: never queued because not connected (degree 0),
    //      or no longer queued because it has been yielded itself,
    //      or no longer queued because all neighbours have been yielded
    //   1...maxPriority: candidates queued with priority (degree - #of yielded neighbours)
    private val priorityPerVertex: IntArray
    private val queue: SimplePriorityQueue<Int>
    private var numLeftToPick: Int

    init {
        var maxPriority = 0
        priorityPerVertex = IntArray(graph.order)
        numLeftToPick = 0
        for (candidate in 0..<graph.order) {
            val degree = graph.degree(candidate)
            if (degree > 0) {
                maxPriority = max(maxPriority, degree)
                priorityPerVertex[candidate] = degree
                numLeftToPick += 1
            }
        }
        queue = SimplePriorityQueue(maxPriority, numLeftToPick)
        for (candidate in 0..<graph.order) {
            val priority = priorityPerVertex[candidate]
            if (priority != 0) {
                queue.put(priority, candidate)
            }
        }
    }

    override fun hasNext(): Boolean {
        return numLeftToPick > 0
    }

    override fun nextInt(): Int {
        while (true) {
            Debug.assert { hasNext() }
            Debug.assert { priorityPerVertex.indices.all { v -> queue.ensure(priorityPerVertex[v], v) } }

            val pick = queue.pop()
            if (priorityPerVertex[pick] != 0) {
                priorityPerVertex[pick] = 0
                numLeftToPick -= 1
                for (v in graph.neighbours(pick)) {
                    val oldPriority = priorityPerVertex[v]
                    if (oldPriority != 0) {
                        val newPriority = oldPriority - 1
                        // Requeue with a more urgent priority or dequeue.
                        // Don't bother to remove the original entry from the queue,
                        // since the vertex will be skipped when popped, and thanks to
                        // numLeftToPick we might not need to pop it at all.
                        priorityPerVertex[v] = newPriority
                        if (newPriority != 0) {
                            queue.put(newPriority, v)
                        } else {
                            numLeftToPick -= 1
                        }
                    }
                }
                return pick
            }
        }
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
        val spliterator = Spliterators.spliteratorUnknownSize(this, characteristics)
        return StreamSupport.intStream(spliterator, false)
    }

    override fun remove() {
        throw NotImplementedError("I'm a read-only iterator")
    }
}
