class RandomGraphGeneratorTest extends org.scalatest.FunSuite {
  test("random_graph") {
    RandomGraphGenerator.new_undirected(2, 0)
    RandomGraphGenerator.new_undirected(3, 0)
    RandomGraphGenerator.new_undirected(3, 1)
    RandomGraphGenerator.new_undirected(3, 2)
    RandomGraphGenerator.new_undirected(4, 0)
    RandomGraphGenerator.new_undirected(4, 1)
    RandomGraphGenerator.new_undirected(4, 2)
    RandomGraphGenerator.new_undirected(4, 3)
    RandomGraphGenerator.new_undirected(4, 4)
    RandomGraphGenerator.new_undirected(4, 5)
  }
}
