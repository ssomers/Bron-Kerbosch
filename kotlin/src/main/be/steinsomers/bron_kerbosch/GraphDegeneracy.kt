package be.steinsomers.bron_kerbosch

internal data class GraphDegeneracyItem(val pick: Vertex, val pickedNeighbours: MutableSet<Vertex>)

internal class GraphDegeneracy(private val graph: UndirectedGraph) : Iterator<GraphDegeneracyItem> {
    // Possible values of priorityPerVertex (after initialization):
    //   0: never queued because not connected (degree 0),
    //      or no longer queued because it has been yielded itself,
    //      or no longer queued because all neighbours have been yielded
    //   1 or more: candidates queued with priority (degree - #of yielded neighbours)
    private val priorityPerVertex = IntArray(graph.order)
    private val queue = SimplePriorityQueue<Vertex>(graph.maxDegree)
    private var numLeftToPick = 0

    init {
        graph.connectedVertices().forEach { v ->
            val priority = graph.degree(v)
            priorityPerVertex[v.index] = priority
            numLeftToPick += 1
            queue.put(priority, v)
        }
    }

    override fun hasNext(): Boolean {
        return numLeftToPick > 0
    }

    override fun next(): GraphDegeneracyItem {
        while (true) {
            Debug.assert { hasNext() }
            Debug.assert { priorityPerVertex.indices.all { v -> queue.ensure(priorityPerVertex[v], Vertex(v)) } }

            val pick = queue.pop()
            if (priorityPerVertex[pick.index] != 0) {
                priorityPerVertex[pick.index] = 0
                val neighbours = graph.neighbours(pick)
                val pickedNeighbours: MutableSet<Vertex> = HashSet(neighbours.size)
                for (v in neighbours) {
                    val oldPriority = priorityPerVertex[v.index]
                    if (oldPriority != 0) {
                        val newPriority = oldPriority - 1
                        // Requeue with a more urgent priority or dequeue.
                        // Don't bother to remove the original entry from the queue,
                        // since the vertex will be skipped when popped, and thanks to
                        // numLeftToPick we might not need to pop it at all.
                        priorityPerVertex[v.index] = newPriority
                        if (newPriority != 0) {
                            queue.put(newPriority, v)
                        } else {
                            // We discount this neighbour already, but logically it will
                            // be (silently) picked only after we yield the current pick.
                            // So it does not belong in the current pickedNeighbours.
                            numLeftToPick -= 1
                        }
                    } else {
                        pickedNeighbours.add(v)
                    }
                }
                numLeftToPick -= 1
                assert(numLeftToPick >= 0)
                Debug.assert { pickedNeighbours.size < graph.degree(pick) }
                return GraphDegeneracyItem(pick = pick, pickedNeighbours = pickedNeighbours)
            }
        }
    }

    private class SimplePriorityQueue<T>(maxPriority: Int) {
        private val stackPerPriority = Array<ArrayList<T>>(size = maxPriority) { _ -> ArrayList() }

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
}
