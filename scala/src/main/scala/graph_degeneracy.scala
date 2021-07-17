import scala.collection.mutable

class degeneracy_ordering[Vertex: Integral] private (
    graph: UndirectedGraph[Vertex],
    priority_per_vertex: Array[Int],
    queue: PriorityQueue[Vertex],
    var num_left_to_pick: Int
) extends Iterator[Vertex] {
  // priority_per_vertex:
  // If priority is 0, vertex was already picked or was always irrelevant (unconnected);
  // otherwise, vertex is still queued and priority = degree + 1 - number of picked neighbours.

  private def priority_of_vertex(v: Vertex): Int = {
    priority_per_vertex(implicitly[Integral[Vertex]].toInt(v))
  }

  private def prioritize_vertex(v: Vertex, p: Int): Unit = {
    priority_per_vertex(implicitly[Integral[Vertex]].toInt(v)) = p
  }

  override def hasNext: Boolean = { num_left_to_pick > 0 }

  override def next(): Vertex = {
    assert(
      priority_per_vertex.zipWithIndex
        .map { case (p, v) => (p, implicitly[Integral[Vertex]].fromInt(v)) }
        .forall { case (p, v) => p == 0 || queue.contains(p, v) }
    )
    var i = queue.pop().get
    while (priority_of_vertex(i) == 0) {
      // v was requeued with a more urgent priority and therefore already picked
      i = queue.pop().get
    }

    prioritize_vertex(i, 0)
    for (v <- graph.neighbours(i)) {
      val old_priority = priority_of_vertex(v)
      if (old_priority != 0) {
        // Since this is an unvisited neighbour of a vertex just being picked,
        // its priority can't be down to the minimum.
        val new_priority = old_priority - 1
        assert(new_priority > 0)
        // Requeue with a more urgent priority, but don't bother to remove
        // the original entry - it will be skipped if it's reached at all.
        prioritize_vertex(v, new_priority)
        queue.put(new_priority, v)
      }
    }
    num_left_to_pick -= 1
    i
  }
}

object degeneracy_ordering {
  def apply[Vertex: Integral](
      graph: UndirectedGraph[Vertex],
      drop: Int
  ): degeneracy_ordering[Vertex] = {
    assert(drop <= 0)
    val order = graph.order()
    var max_priority = 0
    val priority_per_vertex = Array.fill[Int](order)(0)
    var num_candidates = 0
    for (i <- 0 until order) {
      val c = implicitly[Integral[Vertex]].fromInt(i)
      val degree = graph.degree(c)
      if (degree > 0) {
        val priority = degree + 1
        max_priority = math.max(max_priority, priority)
        priority_per_vertex(i) = priority
        num_candidates += 1
      }
    }
    val queue = new PriorityQueue[Vertex](max_priority)
    for (i <- 0 until order) {
      val c = implicitly[Integral[Vertex]].fromInt(i)
      val priority = priority_per_vertex(i)
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

class PriorityQueue[Vertex](max_priority: Int) {
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
