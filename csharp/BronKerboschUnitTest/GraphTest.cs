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
            UndirectedGraph.random_undirected_graph(2, 0);
            UndirectedGraph.random_undirected_graph(3, 0);
            UndirectedGraph.random_undirected_graph(3, 1);
            UndirectedGraph.random_undirected_graph(3, 2);
            UndirectedGraph.random_undirected_graph(4, 0);
            UndirectedGraph.random_undirected_graph(4, 1);
            UndirectedGraph.random_undirected_graph(4, 2);
            UndirectedGraph.random_undirected_graph(4, 3);
            UndirectedGraph.random_undirected_graph(4, 4);
            UndirectedGraph.random_undirected_graph(4, 5);
        }
    }
}