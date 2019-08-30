import base.Vertex

trait UndirectedGraph {
  def order(): Int

  def size(): Int

  def degree(node: Vertex): Int

  def neighbours(node: Vertex): Set[Vertex]

  def connected_vertices(): Iterable[Vertex]
}
