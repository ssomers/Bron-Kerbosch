import base.Vertex

import scala.collection.mutable

class degeneracy_ordering private (graph: UndirectedGraph,
                                   priority_per_vertex: Array[Int],
                                   queue: PriorityQueue,
                                   var num_left_to_pick: Int)
    extends Iterator[Vertex] {
  // priority_per_vertex:
  // If priority is 0, vertex was already picked or was always irrelevant (unconnected);
  // otherwise, vertex is still queued and priority = degree + 1 - number of picked neighbours.

  override def hasNext: Boolean = { num_left_to_pick > 0 }
  override def next(): Vertex = {
    assert(
      priority_per_vertex.zipWithIndex
        .forall { case (p, v) => p == 0 || queue.contains(p, v) }
    )
    var i = queue.pop().get
    while (priority_per_vertex(i) == 0) {
      // v was requeued with a more urgent priority and therefore already picked
      i = queue.pop().get
    }

    priority_per_vertex(i) = 0
    for (v <- graph.neighbours(i)) {
      val old_priority = priority_per_vertex(v)
      if (old_priority != 0) {
        // Since this is an unvisited neighbour of a vertex just being picked,
        // its priority can't be down to the minimum.
        val new_priority = old_priority - 1
        assert(new_priority > 0)
        // Requeue with a more urgent priority, but don't bother to remove
        // the original entry - it will be skipped if it's reached at all.
        priority_per_vertex(v) = new_priority
        queue.put(new_priority, v)
      }
    }
    num_left_to_pick -= 1
    i
  }
}

object degeneracy_ordering {
  def apply(graph: UndirectedGraph, drop: Int): degeneracy_ordering = {
    assert(drop <= 0)
    val order = graph.order()
    var max_priority = 0
    val priority_per_vertex = Array.fill[Int](order)(0)
    var num_candidates = 0
    for (c <- 0 until order) {
      val degree = graph.degree(c)
      if (degree > 0) {
        val priority = degree + 1
        max_priority = math.max(max_priority, priority)
        priority_per_vertex(c) = priority
        num_candidates += 1
      }
    }
    val queue = new PriorityQueue(max_priority)
    for (c <- 0 until order) {
      val priority = priority_per_vertex(c)
      if (priority != 0) {
        queue.put(priority, c)
      }
    }
    new degeneracy_ordering(
      graph,
      priority_per_vertex,
      queue,
      num_candidates + drop
    )
  }
}

class PriorityQueue(max_priority: Int) {
  val stack_per_priority: Array[mutable.ArrayBuffer[Vertex]] =
    Array.fill(max_priority) { new mutable.ArrayBuffer[Vertex] }

  def put(priority: Int, v: Vertex): Unit = {
    stack_per_priority(priority - 1) += v
  }

  def pop(): Option[Vertex] = {
    for (stack <- stack_per_priority) {
      val element = stack.lastOption
      if (element.isDefined) {
        stack.remove(stack.length - 1)
        return element
      }
    }
    None
  }

  def contains(priority: Int, v: Vertex): Boolean = {
    stack_per_priority(priority - 1).contains(v)
  }
}
