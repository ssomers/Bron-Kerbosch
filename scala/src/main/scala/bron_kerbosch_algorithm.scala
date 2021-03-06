import base.Vertex

import scala.collection.immutable

trait bron_kerbosch_algorithm {
  type Clique = immutable.Iterable[Vertex]
  def explore(graph: UndirectedGraph, reporter: Clique => Unit): Unit
}
