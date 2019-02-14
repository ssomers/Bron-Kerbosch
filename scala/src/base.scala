package object base {
  type Vertex = Int
  type Clique = Seq[Vertex]
  type Adjacencies = IndexedSeq[Set[Vertex]]

  def intersect(vset1: Set[Vertex], vset2: Set[Vertex]): Set[Vertex] = {
    if (vset1.size <= vset2.size)
      vset1 & vset2
    else
      vset2 & vset1
  }
}
