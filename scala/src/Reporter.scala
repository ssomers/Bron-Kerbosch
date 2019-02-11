import base.Clique

trait Reporter {
  def record(clique: Clique): Unit
}
