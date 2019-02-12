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

  override def connected_nodes(): Set[Vertex] = {
    neighboursByNode.zipWithIndex
      .filter { case (n, _) => n.nonEmpty }
      .map { case (_, v) => v }
      .toSet
  }
}
