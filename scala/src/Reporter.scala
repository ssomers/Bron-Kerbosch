import base.Vertex

trait Reporter {
  def record(clique: Iterable[Vertex]): Unit
}
