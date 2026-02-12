using BronKerbosch;
using NUnit.Framework;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

namespace BronKerboschUnitTest
{
    public class BronKerboschHashTest : BronKerboschTestTemplate<HashSet<Vertex>, HashSetMgr> { }
    public class BronKerboschSortedTest : BronKerboschTestTemplate<SortedSet<Vertex>, SortedSetMgr> { }

    public class BronKerboschTestTemplate<TVertexSet, TVertexSetMgr>
        where TVertexSet : ISet<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        private static void Bk(int[][] adjacencies, int[][] expectedCliques)
        {
            var adjacencies2 = adjacencies.Select(neighbours => TVertexSetMgr.From(neighbours.Select(i => Vertex.Nth(i))))
                                 .ToImmutableArray();
            var expectedCliques2 = expectedCliques.Select(clique => clique.Select(i => Vertex.Nth(i)).ToArray()).ToArray();
            var graph = new UndirectedGraph<TVertexSet, TVertexSetMgr>(adjacencies2);
            foreach (var funcIndex in Enumerable.Range(0, Portfolio.FuncNames.Length))
            {
                var consumer = new CliqueCollector();
                Portfolio.Explore(funcIndex, graph, consumer);
                var result = consumer.List();
                Assert.That(result.Count, Is.EqualTo(expectedCliques2.Length));
                Portfolio.SortCliques(result);
                foreach ((var reportedClique, var i) in result.Select((v, i) => (v, i)))
                    Assert.That(reportedClique.SequenceEqual(expectedCliques2[i]));
            }
        }

        [Test]
        public void TestOrder0()
        {
            Bk(adjacencies: [],
               expectedCliques: []);
        }

        [Test]
        public void TestOrder1()
        {
            Bk(adjacencies: [[]],
               expectedCliques: []);
        }

        [Test]
        public void TestOrder2_Isolated()
        {
            Bk(adjacencies: [[], []],
               expectedCliques: []);
        }

        [Test]
        public void TestOrder2_Connected()
        {
            Bk(adjacencies: [[1], [0]],
               expectedCliques: [[0, 1]]);
        }

        [Test]
        public void TestOrder3_Size1_Left()
        {
            Bk(adjacencies: [[1], [0], []],
               expectedCliques: [[0, 1]]);
        }

        [Test]
        public void TestOrder3_Size1_Long()
        {
            Bk(adjacencies: [[2], [], [0]],
               expectedCliques: [[0, 2]]);
        }

        [Test]
        public void TestOrder3_Size1_Right()
        {
            Bk(adjacencies: [[], [2], [1]],
               expectedCliques: [[1, 2]]);
        }

        [Test]
        public void TestOrder3_Size2()
        {
            Bk(adjacencies: [[1], [0, 2], [1]],
               expectedCliques: [[0, 1], [1, 2]]);
        }

        [Test]
        public void TestOrder3_Size3()
        {
            Bk(adjacencies: [[1, 2], [0, 2], [0, 1]],
               expectedCliques: [[0, 1, 2]]);
        }

        [Test]
        public void TestOrder4_Size2()
        {
            Bk(adjacencies: [[1], [0], [3], [2]],
               expectedCliques: [[0, 1], [2, 3]]);
        }

        [Test]
        public void TestOrder4_Size3_Bus()
        {
            Bk(adjacencies: [[1], [0, 2], [1, 3], [2]],
               expectedCliques: [[0, 1], [1, 2], [2, 3]]);
        }

        [Test]
        public void TestOrder4_Size3_Star()
        {
            Bk(adjacencies: [[1, 2, 3], [0], [0], [0]],
               expectedCliques: [[0, 1], [0, 2], [0, 3]]);
        }

        [Test]
        public void TestOrder4_Size4_p()
        {
            Bk(adjacencies: [[1], [0, 2, 3], [1, 3], [1, 2]],
               expectedCliques: [[0, 1], [1, 2, 3]]);
        }

        [Test]
        public void TestOrder4_Size4_Square()
        {
            Bk(adjacencies: [[1, 3], [0, 2], [1, 3], [0, 2]],
               expectedCliques: [[0, 1], [0, 3], [1, 2], [2, 3]]);
        }

        [Test]
        public void TestOrder4_Size5()
        {
            Bk(adjacencies: [[1, 2, 3], [0, 2], [0, 1, 3], [0, 2]],
               expectedCliques: [[0, 1, 2], [0, 2, 3]]);
        }

        [Test]
        public void TestOrder4_Size6()
        {
            Bk(adjacencies: [[1, 2, 3], [0, 2, 3], [0, 1, 3], [0, 1, 2]],
               expectedCliques: [[0, 1, 2, 3]]);
        }

        [Test]
        public void TestOrder4_Size6_Penultimate()
        {
            Bk(adjacencies: [[1, 2, 3, 4], [0, 2, 3, 4], [0, 1, 3, 4], [0, 1, 2], [0, 1, 2]],
               expectedCliques: [[0, 1, 2, 3], [0, 1, 2, 4]]);
        }

        [Test]
        public void TestSample()
        {
            Bk(adjacencies:
                [
                    [],
                    [2, 3, 4],
                    [1, 3, 4, 5],
                    [1, 2, 4, 5],
                    [1, 2, 3],
                    [2, 3, 6, 7],
                    [5, 7],
                    [5, 6]
                ],
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
            Bk(adjacencies:
                [
                    [1, 2, 3, 4, 6, 7],
                    [0, 3, 6, 7, 8, 9],
                    [0, 3, 5, 7, 8, 9],
                    [0, 1, 2, 4, 9],
                    [0, 3, 6, 7, 9],
                    [2, 6],
                    [0, 1, 4, 5, 9],
                    [0, 1, 2, 4, 9],
                    [1, 2],
                    [1, 2, 3, 4, 6, 7]
                ],
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
