using Graph;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace BronKerboschUnitTest
{
    [TestClass]
    public class GraphTest
    {
        [TestMethod]
        public void random_undirected_graph()
        {
            //random.seed(19680516)
            UndirectedGraph.GenerateRandom(2, 0);
            UndirectedGraph.GenerateRandom(3, 0);
            UndirectedGraph.GenerateRandom(3, 1);
            UndirectedGraph.GenerateRandom(3, 2);
            UndirectedGraph.GenerateRandom(4, 0);
            UndirectedGraph.GenerateRandom(4, 1);
            UndirectedGraph.GenerateRandom(4, 2);
            UndirectedGraph.GenerateRandom(4, 3);
            UndirectedGraph.GenerateRandom(4, 4);
            UndirectedGraph.GenerateRandom(4, 5);
        }
    }
}