import base.Vertex

trait bron_kerbosch_algorithm {
  type Clique = Seq[Vertex]
  type Cliques = Seq[Clique]
  def explore(graph: UndirectedGraph): Cliques
}
