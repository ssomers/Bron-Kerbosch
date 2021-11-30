import scala.collection.mutable

object util {
  def diff[Vertex](
      vset1: Set[Vertex],
      vset2: mutable.Set[Vertex]
  ): Set[Vertex] = {
    if (vset1.size <= vset2.size) {
      // Same code as `diff` on an immutable set
      vset1.foldLeft(Set.empty[Vertex])((result, v) =>
        if (vset2 contains v) result else result + v
      )
    } else
      vset1 removedAll vset2
  }

  def intersect[Vertex](
      vset1: Set[Vertex],
      vset2: Set[Vertex]
  ): Set[Vertex] = {
    if (vset1.size <= vset2.size)
      vset1 & vset2
    else
      vset2 & vset1
  }

  def intersect[Vertex](
      vset1: mutable.Set[Vertex],
      vset2: Set[Vertex]
  ): Set[Vertex] = {
    if (vset1.size <= vset2.size)
      vset1.toSet & vset2
    else
      vset2 & vset1
  }

  def intersection_size[Vertex](
      vset1: mutable.Set[Vertex],
      vset2: Set[Vertex]
  ): Int = {
    if (vset1.size <= vset2.size)
      vset1.count(v => vset2 contains v)
    else
      vset2.count(v => vset1 contains v)
  }

  def are_disjoint[Vertex](
      vset1: mutable.Set[Vertex],
      vset2: mutable.Set[Vertex]
  ): Boolean = {
    if (vset1.size <= vset2.size)
      vset1.forall(v => !(vset2 contains v))
    else
      vset2.forall(v => !(vset1 contains v))
  }

  def are_disjoint[Vertex](
      vset1: Set[Vertex],
      vset2: Set[Vertex]
  ): Boolean = {
    if (vset1.size <= vset2.size)
      vset1.forall(v => !(vset2 contains v))
    else
      vset2.forall(v => !(vset1 contains v))
  }

  def are_disjoint[Vertex](
      vset1: mutable.Set[Vertex],
      vset2: Set[Vertex]
  ): Boolean = {
    if (vset1.size <= vset2.size)
      vset1.forall(v => !(vset2 contains v))
    else
      vset2.forall(v => !(vset1 contains v))
  }
}
