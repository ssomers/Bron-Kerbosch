object util {
  def intersect[Vertex](vset1: Set[Vertex], vset2: Set[Vertex]): Set[Vertex] = {
    if (vset1.size <= vset2.size)
      vset1 & vset2
    else
      vset2 & vset1
  }

  def intersection_size[Vertex](vset1: Set[Vertex], vset2: Set[Vertex]): Int = {
    if (vset1.size <= vset2.size)
      vset1.count(v => vset2.contains(v))
    else
      vset2.count(v => vset1.contains(v))
  }

  def are_disjoint[Vertex](vset1: Set[Vertex], vset2: Set[Vertex]): Boolean = {
    if (vset1.size <= vset2.size)
      vset1.forall(v => !vset2.contains(v))
    else
      vset2.forall(v => !vset1.contains(v))
  }
}
