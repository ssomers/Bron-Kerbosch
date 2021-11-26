using BronKerbosch;
using NUnit.Framework;
using System.Collections.Immutable;
using System.Linq;

namespace BronKerboschUnitTest
{
    public class BronKerboschTest
    {
        private static void Bk(int[][] adjacencies, int[][] cliques)
        {
            var adjacencies2 = adjacencies.Select(neighbours => neighbours.Select(i => Vertex.nth(i)).ToHashSet())
                                 .ToImmutableArray();
            var cliques2 = cliques.Select(clique => clique.Select(i => Vertex.nth(i)).ToArray()).ToArray();
            var graph = new UndirectedGraph(adjacencies2);
            foreach (var funcIndex in Enumerable.Range(0, Portfolio.FuncNames.Length))
            {
                var reporter = new SimpleReporter();
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
            Bk(adjacencies: new int[][] { },
               cliques: new int[][] { });
        }

        [Test]
        public void TestOrder1()
        {
            Bk(adjacencies: new[] { new int[] { } },
               cliques: new int[][] { });
        }

        [Test]
        public void TestOrder2_Isolated()
        {
            Bk(adjacencies: new[] { new int[] { }, new int[] { } },
               cliques: new int[][] { });
        }

        [Test]
        public void TestOrder2_Connected()
        {
            Bk(adjacencies: new[] { new int[] { 1 }, new int[] { 0 } },
               cliques: new[] { new int[] { 0, 1 } });
        }

        [Test]
        public void TestOrder3_Size1_Left()
        {
            Bk(adjacencies: new[] { new int[] { 1 }, new int[] { 0 }, new int[] { } },
               cliques: new[] { new int[] { 0, 1 } });
        }

        [Test]
        public void TestOrder3_Size1_Long()
        {
            Bk(adjacencies: new[] { new int[] { 2 }, new int[] { }, new int[] { 0 } },
               cliques: new[] { new int[] { 0, 2 } });
        }

        [Test]
        public void TestOrder3_Size1_Right()
        {
            Bk(adjacencies: new[] { new int[] { }, new int[] { 2 }, new int[] { 1 } },
               cliques: new[] { new int[] { 1, 2 } });
        }

        [Test]
        public void TestOrder3_Size2()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1},
                    new int[] {0, 2},
                    new int[] {1}
                },
               cliques: new[] { new int[] { 0, 1 }, new int[] { 1, 2 } });
        }

        [Test]
        public void TestOrder3_Size3()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1, 2},
                    new int[] {0, 2},
                    new int[] {0, 1}
                },
               cliques: new[] { new int[] { 0, 1, 2 } });
        }

        [Test]
        public void TestOrder4_Size2()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1}, new int[] {0},
                    new int[] {3}, new int[] {2}
                },
               cliques: new[] { new int[] { 0, 1 }, new int[] { 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size3_Bus()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1}, new int[] {0, 2},
                    new int[] {1, 3}, new int[] {2}
                },
               cliques: new[] { new int[] { 0, 1 }, new int[] { 1, 2 }, new int[] { 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size3_Star()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1, 2, 3}, new int[] {0},
                    new int[] {0}, new int[] {0}
                },
               cliques: new[] { new int[] { 0, 1 }, new int[] { 0, 2 }, new int[] { 0, 3 } });
        }

        [Test]
        public void TestOrder4_Size4_p()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1}, new int[] {0, 2, 3},
                    new int[] {1, 3}, new int[] {1, 2}
                },
               cliques: new[] { new int[] { 0, 1 }, new int[] { 1, 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size4_Square()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1, 3}, new int[] {0, 2},
                    new int[] {1, 3}, new int[] {0, 2}
                },
               cliques: new[]
                {
                    new int[] {0, 1}, new int[] {0, 3},
                    new int[] {1, 2}, new int[] {2, 3}
                });
        }

        [Test]
        public void TestOrder4_Size5()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1, 2, 3}, new int[] {0, 2},
                    new int[] {0, 1, 3}, new int[] {0, 2}
                },
               cliques: new[] { new int[] { 0, 1, 2 }, new int[] { 0, 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size6()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1, 2, 3}, new int[] {0, 2, 3},
                    new int[] {0, 1, 3}, new int[] {0, 1, 2}
                },
               cliques: new[] { new int[] { 0, 1, 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size6_Penultimate()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1, 2, 3, 4}, new int[] {0, 2, 3, 4},
                    new int[] {0, 1, 3, 4}, new int[] {0, 1, 2},
                    new int[] {0, 1, 2}
                },
               cliques: new[] { new int[] { 0, 1, 2, 3 }, new int[] { 0, 1, 2, 4 } });
        }

        [Test]
        public void TestSample()
        {
            Bk(adjacencies: new[]
                {
                    new int[] { },
                    new int[] {2, 3, 4},
                    new int[] {1, 3, 4, 5},
                    new int[] {1, 2, 4, 5},
                    new int[] {1, 2, 3},
                    new int[] {2, 3, 6, 7},
                    new int[] {5, 7},
                    new int[] {5, 6}
                },
               cliques: new[]
                {
                    new int[] {1, 2, 3, 4},
                    new int[] {2, 3, 5},
                    new int[] {5, 6, 7}
                });
        }

        [Test]
        public void TestBigger()
        {
            Bk(adjacencies: new[]
                {
                    new int[] {1, 2, 3, 4, 6, 7},
                    new int[] {0, 3, 6, 7, 8, 9},
                    new int[] {0, 3, 5, 7, 8, 9},
                    new int[] {0, 1, 2, 4, 9},
                    new int[] {0, 3, 6, 7, 9},
                    new int[] {2, 6},
                    new int[] {0, 1, 4, 5, 9},
                    new int[] {0, 1, 2, 4, 9},
                    new int[] {1, 2},
                    new int[] {1, 2, 3, 4, 6, 7}
                },
               cliques: new[]
                {
                    new int[] {0, 1, 3},
                    new int[] {0, 1, 6},
                    new int[] {0, 1, 7},
                    new int[] {0, 2, 3},
                    new int[] {0, 2, 7},
                    new int[] {0, 3, 4},
                    new int[] {0, 4, 6},
                    new int[] {0, 4, 7},
                    new int[] {1, 3, 9},
                    new int[] {1, 6, 9},
                    new int[] {1, 7, 9},
                    new int[] {1, 8},
                    new int[] {2, 3, 9},
                    new int[] {2, 5},
                    new int[] {2, 7, 9},
                    new int[] {2, 8},
                    new int[] {3, 4, 9},
                    new int[] {4, 6, 9},
                    new int[] {4, 7, 9},
                    new int[] {5, 6}
                });
        }
    }
}
