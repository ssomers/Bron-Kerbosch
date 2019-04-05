import base.Vertex

object util {
  def intersect(vset1: Set[Vertex], vset2: Set[Vertex]): Set[Vertex] = {
    if (vset1.size <= vset2.size)
      vset1 & vset2
    else
      vset2 & vset1
  }

  def is_disjoint(vset1: Set[Vertex], vset2: Set[Vertex]): Boolean = {
    if (vset1.size <= vset2.size)
      vset1.forall(v => !vset2.contains(v))
    else
      vset2.forall(v => !vset1.contains(v))
  }
}
