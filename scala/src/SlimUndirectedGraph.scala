import base.{Adjacencies, Vertex}

import scala.collection.mutable

class SlimUndirectedGraph(neighboursByNode: Adjacencies)
    extends UndirectedGraph {

  override def order(): Int = {
    neighboursByNode.length
  }

  override def size(): Int = {
    val total = neighboursByNode.map(n => n.size).sum
    require(total % 2 == 0)
    total / 2
  }

  override def degree(node: Vertex): Int = {
    neighboursByNode(node).size
  }

  override def neighbours(node: Vertex): Set[Vertex] = {
    neighboursByNode(node)
  }

  override def connected_nodes(): mutable.Set[Vertex] = {
    val set = neighboursByNode.zipWithIndex
      .filter { case (n, v) => !n.isEmpty }
      .map { case (n, v) => v }
      .toSet
    mutable.Set.empty[Vertex] ++= set
  }
}
