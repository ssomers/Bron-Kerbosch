using BronKerboschStudy;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;

[TestClass]
public class RandomGraphTest
{
    [TestMethod]
    public void random_undirected_graph()
    {
        var random = new Random(19680516);
        RandomUndirectedGraph.Generate(random, 2, 0);
        RandomUndirectedGraph.Generate(random, 3, 0);
        RandomUndirectedGraph.Generate(random, 3, 1);
        RandomUndirectedGraph.Generate(random, 3, 2);
        RandomUndirectedGraph.Generate(random, 4, 0);
        RandomUndirectedGraph.Generate(random, 4, 1);
        RandomUndirectedGraph.Generate(random, 4, 2);
        RandomUndirectedGraph.Generate(random, 4, 3);
        RandomUndirectedGraph.Generate(random, 4, 4);
        RandomUndirectedGraph.Generate(random, 4, 5);
    }
}
