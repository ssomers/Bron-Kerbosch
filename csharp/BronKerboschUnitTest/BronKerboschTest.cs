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
        where TVertexSet : IEnumerable<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        private static void Bk(int[][] adjacencies, int[][] cliques)
        {
            var adjacencies2 = adjacencies.Select(neighbours => TVertexSetMgr.From(neighbours.Select(i => Vertex.Nth(i))))
                                 .ToImmutableArray();
            var cliques2 = cliques.Select(clique => clique.Select(i => Vertex.Nth(i)).ToArray()).ToArray();
            var graph = new UndirectedGraph<TVertexSet, TVertexSetMgr>(adjacencies2);
            foreach (var funcIndex in Enumerable.Range(0, Portfolio.FuncNames.Length))
            {
                var reporter = new CollectingReporter();
                Portfolio.Explore(funcIndex, graph, reporter);
                Assert.AreEqual(cliques2.Length, reporter.Cliques.Count);
                Portfolio.SortCliques(reporter.Cliques);
                foreach ((var reportedClique, var i) in reporter.Cliques.Select((v, i) => (v, i)))
                    Assert.That(reportedClique.SequenceEqual(cliques2[i]));
            }
        }

        [Test]
        public void TestOrder0()
        {
            Bk(adjacencies: [],
               cliques: []);
        }

        [Test]
        public void TestOrder1()
        {
            Bk(adjacencies: [[]],
               cliques: []);
        }

        [Test]
        public void TestOrder2_Isolated()
        {
            Bk(adjacencies: [[], []],
               cliques: []);
        }

        [Test]
        public void TestOrder2_Connected()
        {
            Bk(adjacencies: [[1], [0]],
               cliques: [[0, 1]]);
        }

        [Test]
        public void TestOrder3_Size1_Left()
        {
            Bk(adjacencies: [[1], [0], []],
               cliques: [[0, 1]]);
        }

        [Test]
        public void TestOrder3_Size1_Long()
        {
            Bk(adjacencies: [[2], [], [0]],
               cliques: [[0, 2]]);
        }

        [Test]
        public void TestOrder3_Size1_Right()
        {
            Bk(adjacencies: [[], [2], [1]],
               cliques: [[1, 2]]);
        }

        [Test]
        public void TestOrder3_Size2()
        {
            Bk(adjacencies: [[1], [0, 2], [1]],
               cliques: [[0, 1], [1, 2]]);
        }

        [Test]
        public void TestOrder3_Size3()
        {
            Bk(adjacencies: [[1, 2], [0, 2], [0, 1]],
               cliques: [[0, 1, 2]]);
        }

        [Test]
        public void TestOrder4_Size2()
        {
            Bk(adjacencies: [[1], [0], [3], [2]],
               cliques: [[0, 1], [2, 3]]);
        }

        [Test]
        public void TestOrder4_Size3_Bus()
        {
            Bk(adjacencies: [[1], [0, 2], [1, 3], [2]],
               cliques: [[0, 1], [1, 2], [2, 3]]);
        }

        [Test]
        public void TestOrder4_Size3_Star()
        {
            Bk(adjacencies: [[1, 2, 3], [0], [0], [0]],
               cliques: [[0, 1], [0, 2], [0, 3]]);
        }

        [Test]
        public void TestOrder4_Size4_p()
        {
            Bk(adjacencies: [[1], [0, 2, 3], [1, 3], [1, 2]],
               cliques: [[0, 1], [1, 2, 3]]);
        }

        [Test]
        public void TestOrder4_Size4_Square()
        {
            Bk(adjacencies: [[1, 3], [0, 2], [1, 3], [0, 2]],
               cliques: [[0, 1], [0, 3], [1, 2], [2, 3]]);
        }

        [Test]
        public void TestOrder4_Size5()
        {
            Bk(adjacencies: [[1, 2, 3], [0, 2], [0, 1, 3], [0, 2]],
               cliques: [[0, 1, 2], [0, 2, 3]]);
        }

        [Test]
        public void TestOrder4_Size6()
        {
            Bk(adjacencies: [[1, 2, 3], [0, 2, 3], [0, 1, 3], [0, 1, 2]],
               cliques: [[0, 1, 2, 3]]);
        }

        [Test]
        public void TestOrder4_Size6_Penultimate()
        {
            Bk(adjacencies: [[1, 2, 3, 4], [0, 2, 3, 4], [0, 1, 3, 4], [0, 1, 2], [0, 1, 2]],
               cliques: [[0, 1, 2, 3], [0, 1, 2, 4]]);
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
               cliques:
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
               cliques:
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
