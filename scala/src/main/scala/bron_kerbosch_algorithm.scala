import base.Vertex

import scala.collection.immutable

trait bron_kerbosch_algorithm {
  type Clique = immutable.Iterable[Vertex]
  type Cliques = immutable.Iterable[Clique]
  def explore(graph: UndirectedGraph): Cliques
}
