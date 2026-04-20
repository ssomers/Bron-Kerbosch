using NUnit.Framework;
using System.Collections.Generic;
using System.Linq;

namespace BronKerbosch.Test
{
    public class BronKerboschHashTest : BronKerboschTestTemplate<HashSet<Vertex>, HashSetMgr> { }
    public class BronKerboschSortedTest : BronKerboschTestTemplate<SortedSet<Vertex>, SortedSetMgr> { }

    public class BronKerboschTestTemplate<TVertexSet, TVertexSetMgr> : LabGraphs<TVertexSet, TVertexSetMgr>
        where TVertexSet : ISet<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        private static void RunSerial(LabGraph<TVertexSet, TVertexSetMgr> g, int[][] expectedCliques)
        {
            Vertex[][] expectedCliques2 = [.. expectedCliques.Select(clique => clique.Select(i => Vertex.Nth(i)).ToArray())];
            foreach (var funcIndex in Enumerable.Range(0, Portfolio.FuncNames.Length))
            {
                var consumer = new CliqueCollector(2);
                Portfolio.Explore(funcIndex, g.Graph, consumer, 1);
                var result = consumer.Cliques;
                Assert.That(result.Count, Is.EqualTo(expectedCliques2.Length));
                Portfolio.SortCliques(result);
                foreach ((var reportedClique, var i) in result.Select((v, i) => (v, i)))
                    Assert.That(reportedClique.SequenceEqual(expectedCliques2[i]));
            }
        }

        private static void RunParallel(LabGraph<TVertexSet, TVertexSetMgr> g, int[][] expectedCliques)
        {
            Vertex[][] expectedCliques2 = [.. expectedCliques.Select(clique => clique.Select(i => Vertex.Nth(i)).ToArray())];
            foreach (var funcIndex in Enumerable.Range(Portfolio.FuncIndexMT, 1))
            {
                var consumer = new CliqueCollector(2);
                Portfolio.Explore(funcIndex, g.Graph, consumer, 16);
                var result = consumer.Cliques;
                Assert.That(result.Count, Is.EqualTo(expectedCliques2.Length));
                Portfolio.SortCliques(result);
                foreach ((var reportedClique, var i) in result.Select((v, i) => (v, i)))
                    Assert.That(reportedClique.SequenceEqual(expectedCliques2[i]));
            }
        }

        private static void Bk(LabGraph<TVertexSet, TVertexSetMgr> g, int[][] expectedCliques)
        {
            RunSerial(g, expectedCliques);
            RunParallel(g, expectedCliques);
        }

        [Test]
        public void TestOrder0()
        {
            Bk(Order0,
               expectedCliques: []);
        }

        [Test]
        public void TestOrder1()
        {
            Bk(Order1,
               expectedCliques: []);
        }

        [Test]
        public void TestOrder2_isolated()
        {
            Bk(Order2_isolated,
               expectedCliques: []);
        }

        [Test]
        public void TestOrder2_connected()
        {
            Bk(Order2_connected,
               expectedCliques: [[0, 1]]);
        }

        [Test]
        public void TestOrder3_size1_left()
        {
            Bk(Order3_size1_left,
               expectedCliques: [[0, 1]]);
        }

        [Test]
        public void TestOrder3_size1_long()
        {
            Bk(Order3_size1_long,
               expectedCliques: [[0, 2]]);
        }

        [Test]
        public void TestOrder3_size1_right()
        {
            Bk(Order3_size1_right,
               expectedCliques: [[1, 2]]);
        }

        [Test]
        public void TestOrder3_size2()
        {
            Bk(Order3_size2,
               expectedCliques: [[0, 1], [1, 2]]);
        }

        [Test]
        public void TestOrder3_size3()
        {
            Bk(Order3_size3,
               expectedCliques: [[0, 1, 2]]);
        }

        [Test]
        public void TestOrder4_size2()
        {
            Bk(Order4_size2,
               expectedCliques: [[0, 1], [2, 3]]);
        }

        [Test]
        public void TestOrder4_size3_bus()
        {
            Bk(Order4_size3_bus,
               expectedCliques: [[0, 1], [1, 2], [2, 3]]);
        }

        [Test]
        public void TestOrder4_size3_star()
        {
            Bk(Order4_size3_star,
               expectedCliques: [[0, 1], [0, 2], [0, 3]]);
        }

        [Test]
        public void TestOrder4_size4_p()
        {
            Bk(Order4_size4_p,
               expectedCliques: [[0, 1], [1, 2, 3]]);
        }

        [Test]
        public void TestOrder4_size4_square()
        {
            Bk(Order4_size4_square,
               expectedCliques: [[0, 1], [0, 3], [1, 2], [2, 3]]);
        }

        [Test]
        public void TestOrder4_size5()
        {
            Bk(Order4_size5,
               expectedCliques: [[0, 1, 2], [0, 2, 3]]);
        }

        [Test]
        public void TestOrder4_size6()
        {
            Bk(Order4_size6,
               expectedCliques: [[0, 1, 2, 3]]);
        }

        [Test]
        public void TestOrder5_size6_penultimate()
        {
            Bk(Order5_size6_penultimate,
               expectedCliques: [[0, 1, 2, 3], [0, 1, 2, 4]]);
        }

        [Test]
        public void TestSample()
        {
            Bk(Sample,
               expectedCliques:
                [
                    [1, 2, 3, 4],
                    [2, 3, 5],
                    [5, 6, 7]
                ]);
        }

        [Test]
        public void TestBigger()
        {
            Bk(Bigger,
               expectedCliques:
                [
                    [0, 1, 3],
                    [0, 1, 6],
                    [0, 1, 7],
                    [0, 2, 3],
                    [0, 2, 7],
                    [0, 3, 4],
                    [0, 4, 6],
                    [0, 4, 7],
                    [1, 3, 9],
                    [1, 6, 9],
                    [1, 7, 9],
                    [1, 8],
                    [2, 3, 9],
                    [2, 5],
                    [2, 7, 9],
                    [2, 8],
                    [3, 4, 9],
                    [4, 6, 9],
                    [4, 7, 9],
                    [5, 6]
                ]);
        }
    }
}
