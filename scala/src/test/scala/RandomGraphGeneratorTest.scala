import scala.util.Random
class RandomGraphGeneratorTest extends org.scalatest.FunSuite {
  test("random_graph") {
    val rng = new Random(19680516L)
    RandomGraphGenerator.new_undirected(rng, 2, 0)
    RandomGraphGenerator.new_undirected(rng, 3, 0)
    RandomGraphGenerator.new_undirected(rng, 3, 1)
    RandomGraphGenerator.new_undirected(rng, 3, 2)
    RandomGraphGenerator.new_undirected(rng, 4, 0)
    RandomGraphGenerator.new_undirected(rng, 4, 1)
    RandomGraphGenerator.new_undirected(rng, 4, 2)
    RandomGraphGenerator.new_undirected(rng, 4, 3)
    RandomGraphGenerator.new_undirected(rng, 4, 4)
    RandomGraphGenerator.new_undirected(rng, 4, 5)
    RandomGraphGenerator.new_undirected(rng, 4, 5) // yes, again
  }
}
