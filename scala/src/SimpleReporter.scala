import base.Clique
import scala.collection.mutable.ArrayBuffer

class SimpleReporter extends Reporter {
  var cliques = ArrayBuffer[Clique]()

  override def record(clique: Clique): Unit = {
    cliques += clique
  }
}
