import base.Vertex

import scala.collection.mutable.ArrayBuffer

class SimpleReporter extends Reporter {
  var cliques: ArrayBuffer[IndexedSeq[Vertex]] =
    ArrayBuffer.empty

  override def record(clique: Iterable[Vertex]): Unit = {
    assert(clique.size > 1)
    cliques += clique.toIndexedSeq
  }
}
