import base.Vertex

import scala.collection.mutable

trait UndirectedGraph {
  def order(): Int

  def size(): Int

  def degree(node: Vertex): Int

  def neighbours(node: Vertex): Set[Vertex]

  def connected_nodes(): mutable.Set[Vertex]
}
