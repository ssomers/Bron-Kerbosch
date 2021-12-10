class SlimUndirectedGraph[Vertex: Integral](
    neighboursByNode: IndexedSeq[Set[Vertex]]
) extends UndirectedGraph[Vertex] {
  override def order(): Int = {
    neighboursByNode.length
  }

  override def size(): Int = {
    val total = neighboursByNode.map(n => n.size).sum
    require(total % 2 == 0)
    total / 2
  }

  override def degree(node: Vertex): Int = {
    neighboursByNode(implicitly[Integral[Vertex]].toInt(node)).size
  }

  override def neighbours(node: Vertex): Set[Vertex] = {
    neighboursByNode(implicitly[Integral[Vertex]].toInt(node))
  }

  override def connected_vertices(): Iterable[Vertex] = {
    neighboursByNode.zipWithIndex
      .filter { case (n, _) => n.nonEmpty }
      .map { case (_, v) => implicitly[Integral[Vertex]].fromInt(v) }
  }

  override def max_degree_vertex(): Vertex = {
    val (_, v) = neighboursByNode.zipWithIndex.maxBy { case (n, _) => n.size }
    implicitly[Integral[Vertex]].fromInt(v)
  }

}
