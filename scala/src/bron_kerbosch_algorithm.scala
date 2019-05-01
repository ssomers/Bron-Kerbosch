import base.Vertex

trait bron_kerbosch_algorithm {
  type Clique = Seq[Vertex]
  type Cliques = Iterable[Clique]
  def explore(graph: UndirectedGraph): Cliques
}
