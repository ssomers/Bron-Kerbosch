using BronKerbosch;
using NUnit.Framework;
using System.Collections.Immutable;
using System.Linq;
using Vertex = System.UInt32;

namespace BronKerboschUnitTest
{
    public class BronKerboschTest
    {
        static void bk(Vertex[][] adjacencies, Vertex[][] cliques)
        {
            var graph = new UndirectedGraph(adjacencies.Select(neighbours => neighbours.ToHashSet())
                .ToImmutableArray());
            foreach (var funcIndex in Enumerable.Range(0, Portfolio.FuncNames.Length))
            {
                var reporter = new SimpleReporter();
                Portfolio.Explore(funcIndex, graph, reporter);
                Assert.That(reporter.Cliques.Count.Equals(cliques.Length));
                Portfolio.SortCliques(reporter.Cliques);
                foreach ((var reportedClique, var i) in reporter.Cliques.Select((v, i) => (v, i)))
                    Assert.That(reportedClique.SequenceEqual(cliques[i]));
            }
        }

        [Test]
        public void TestOrder0()
        {
            bk(adjacencies: new Vertex[][] { },
               cliques: new Vertex[][] { });
        }

        [Test]
        public void TestOrder1()
        {
            bk(adjacencies: new[] { new Vertex[] { } },
               cliques: new Vertex[][] { });
        }

        [Test]
        public void TestOrder2_Isolated()
        {
            bk(adjacencies: new[] { new Vertex[] { }, new Vertex[] { } },
               cliques: new Vertex[][] { });
        }

        [Test]
        public void TestOrder2_Connected()
        {
            bk(adjacencies: new[] { new Vertex[] { 1 }, new Vertex[] { 0 } },
               cliques: new[] { new Vertex[] { 0, 1 } });
        }

        [Test]
        public void TestOrder3_Size1_Left()
        {
            bk(adjacencies: new[] { new Vertex[] { 1 }, new Vertex[] { 0 }, new Vertex[] { } },
               cliques: new[] { new Vertex[] { 0, 1 } });
        }

        [Test]
        public void TestOrder3_Size1_Long()
        {
            bk(adjacencies: new[] { new Vertex[] { 2 }, new Vertex[] { }, new Vertex[] { 0 } },
               cliques: new[] { new Vertex[] { 0, 2 } });
        }

        [Test]
        public void TestOrder3_Size1_Right()
        {
            bk(adjacencies: new[] { new Vertex[] { }, new Vertex[] { 2 }, new Vertex[] { 1 } },
               cliques: new[] { new Vertex[] { 1, 2 } });
        }

        [Test]
        public void TestOrder3_Size2()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1},
                    new Vertex[] {0, 2},
                    new Vertex[] {1}
                },
               cliques: new[] { new Vertex[] { 0, 1 }, new Vertex[] { 1, 2 } });
        }

        [Test]
        public void TestOrder3_Size3()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1, 2},
                    new Vertex[] {0, 2},
                    new Vertex[] {0, 1}
                },
               cliques: new[] { new Vertex[] { 0, 1, 2 } });
        }

        [Test]
        public void TestOrder4_Size2()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1}, new Vertex[] {0},
                    new Vertex[] {3}, new Vertex[] {2}
                },
               cliques: new[] { new Vertex[] { 0, 1 }, new Vertex[] { 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size3_Bus()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1}, new Vertex[] {0, 2},
                    new Vertex[] {1, 3}, new Vertex[] {2}
                },
               cliques: new[] { new Vertex[] { 0, 1 }, new Vertex[] { 1, 2 }, new Vertex[] { 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size3_Star()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1, 2, 3}, new Vertex[] {0},
                    new Vertex[] {0}, new Vertex[] {0}
                },
               cliques: new[] { new Vertex[] { 0, 1 }, new Vertex[] { 0, 2 }, new Vertex[] { 0, 3 } });
        }

        [Test]
        public void TestOrder4_Size4_p()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1}, new Vertex[] {0, 2, 3},
                    new Vertex[] {1, 3}, new Vertex[] {1, 2}
                },
               cliques: new[] { new Vertex[] { 0, 1 }, new Vertex[] { 1, 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size4_Square()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1, 3}, new Vertex[] {0, 2},
                    new Vertex[] {1, 3}, new Vertex[] {0, 2}
                },
               cliques: new[]
                {
                    new Vertex[] {0, 1}, new Vertex[] {0, 3},
                    new Vertex[] {1, 2}, new Vertex[] {2, 3}
                });
        }

        [Test]
        public void TestOrder4_Size5()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1, 2, 3}, new Vertex[] {0, 2},
                    new Vertex[] {0, 1, 3}, new Vertex[] {0, 2}
                },
               cliques: new[] { new Vertex[] { 0, 1, 2 }, new Vertex[] { 0, 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size6()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1, 2, 3}, new Vertex[] {0, 2, 3},
                    new Vertex[] {0, 1, 3}, new Vertex[] {0, 1, 2}
                },
               cliques: new[] { new Vertex[] { 0, 1, 2, 3 } });
        }

        [Test]
        public void TestOrder4_Size6_Penultimate()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1, 2, 3, 4}, new Vertex[] {0, 2, 3, 4},
                    new Vertex[] {0, 1, 3, 4}, new Vertex[] {0, 1, 2},
                    new Vertex[] {0, 1, 2}
                },
               cliques: new[] { new Vertex[] { 0, 1, 2, 3 }, new Vertex[] { 0, 1, 2, 4 } });
        }

        [Test]
        public void TestSample()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] { },
                    new Vertex[] {2, 3, 4},
                    new Vertex[] {1, 3, 4, 5},
                    new Vertex[] {1, 2, 4, 5},
                    new Vertex[] {1, 2, 3},
                    new Vertex[] {2, 3, 6, 7},
                    new Vertex[] {5, 7},
                    new Vertex[] {5, 6}
                },
               cliques: new[]
                {
                    new Vertex[] {1, 2, 3, 4},
                    new Vertex[] {2, 3, 5},
                    new Vertex[] {5, 6, 7}
                });
        }

        [Test]
        public void TestBigger()
        {
            bk(adjacencies: new[]
                {
                    new Vertex[] {1, 2, 3, 4, 6, 7},
                    new Vertex[] {0, 3, 6, 7, 8, 9},
                    new Vertex[] {0, 3, 5, 7, 8, 9},
                    new Vertex[] {0, 1, 2, 4, 9},
                    new Vertex[] {0, 3, 6, 7, 9},
                    new Vertex[] {2, 6},
                    new Vertex[] {0, 1, 4, 5, 9},
                    new Vertex[] {0, 1, 2, 4, 9},
                    new Vertex[] {1, 2},
                    new Vertex[] {1, 2, 3, 4, 6, 7}
                },
               cliques: new[]
                {
                    new Vertex[] {0, 1, 3},
                    new Vertex[] {0, 1, 6},
                    new Vertex[] {0, 1, 7},
                    new Vertex[] {0, 2, 3},
                    new Vertex[] {0, 2, 7},
                    new Vertex[] {0, 3, 4},
                    new Vertex[] {0, 4, 6},
                    new Vertex[] {0, 4, 7},
                    new Vertex[] {1, 3, 9},
                    new Vertex[] {1, 6, 9},
                    new Vertex[] {1, 7, 9},
                    new Vertex[] {1, 8},
                    new Vertex[] {2, 3, 9},
                    new Vertex[] {2, 5},
                    new Vertex[] {2, 7, 9},
                    new Vertex[] {2, 8},
                    new Vertex[] {3, 4, 9},
                    new Vertex[] {4, 6, 9},
                    new Vertex[] {4, 7, 9},
                    new Vertex[] {5, 6}
                });
        }
    }
}
