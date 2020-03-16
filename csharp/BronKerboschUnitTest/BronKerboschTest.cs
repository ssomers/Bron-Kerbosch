using BronKerbosch;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Collections.Immutable;
using System.Linq;
using Vertex = System.UInt32;

namespace BronKerboschUnitTest
{
    [TestClass]
    public class BronKerboschTest
    {
        static void bk(Vertex[][] adjacencies, Vertex[][] expected_cliques)
        {
            var graph = new UndirectedGraph(adjacencies.Select(neighbours => neighbours.ToHashSet()).ToImmutableArray());
            foreach (int func_index in Enumerable.Range(0, Portfolio.FUNC_NAMES.Length))
            {
                var reporter = new SimpleReporter();
                Portfolio.Explore(func_index, graph, reporter);
                Assert.AreEqual(expected_cliques.Length, reporter.Cliques.Count);
                Portfolio.SortCliques(reporter.Cliques);
                foreach ((var clique, int i) in reporter.Cliques.Select((v, i) => (v, i)))
                    Assert.IsTrue(clique.SequenceEqual(expected_cliques[i]));
            }
        }

        [TestMethod]
        public void TestOrder0()
        {
            bk(new Vertex[][] { },
               new Vertex[][] { });
        }

        [TestMethod]
        public void TestOrder1()
        {
            bk(new Vertex[][] { new Vertex[] { } },
               new Vertex[][] { });
        }

        [TestMethod]
        public void TestOrder2_Isolated()
        {
            bk(new Vertex[][] { new Vertex[] { }, new Vertex[] { } },
               new Vertex[][] { });
        }

        [TestMethod]
        public void TestOrder2_Connected()
        {
            bk(new Vertex[][] { new Vertex[] { 1 }, new Vertex[] { 0 } },
               new Vertex[][] { new Vertex[] { 0, 1 } });
        }

        [TestMethod]
        public void TestOrder3_Size1_Left()
        {
            bk(new Vertex[][] { new Vertex[] { 1 }, new Vertex[] { 0 }, new Vertex[] { } },
               new Vertex[][] { new Vertex[] { 0, 1 } });
        }

        [TestMethod]
        public void TestOrder3_Size1_Long()
        {
            bk(new Vertex[][] { new Vertex[] { 2 }, new Vertex[] { }, new Vertex[] { 0 } },
               new Vertex[][] { new Vertex[] { 0, 2 } });
        }

        [TestMethod]
        public void TestOrder3_Size1_Right()
        {
            bk(new Vertex[][] { new Vertex[] { }, new Vertex[] { 2 }, new Vertex[] { 1 } },
               new Vertex[][] { new Vertex[] { 1, 2 } });
        }

        [TestMethod]
        public void TestOrder3_Size2()
        {
            bk(new Vertex[][] { new Vertex[] { 1 },
                                new Vertex[] { 0, 2 },
                                new Vertex[] { 1 } },
               new Vertex[][] { new Vertex[] { 0, 1 }, new Vertex[] { 1, 2 } });
        }

        [TestMethod]
        public void TestOrder3_Size3()
        {
            bk(new Vertex[][] { new Vertex[] { 1, 2 },
                                new Vertex[] { 0, 2 },
                                new Vertex[] { 0, 1 } },
               new Vertex[][] { new Vertex[] { 0, 1, 2 } });
        }

        [TestMethod]
        public void TestOrder4_Size2()
        {
            bk(new Vertex[][] { new Vertex[] { 1 }, new Vertex[] { 0 },
                                new Vertex[] { 3 }, new Vertex[] { 2 } },
               new Vertex[][] { new Vertex[] { 0, 1 }, new Vertex[] { 2, 3 } });
        }

        [TestMethod]
        public void TestOrder4_Size3_Bus()
        {
            bk(new Vertex[][] { new Vertex[] { 1 }, new Vertex[] { 0, 2 },
                                new Vertex[] { 1, 3 }, new Vertex[] { 2 } },
               new Vertex[][] { new Vertex[] { 0, 1 }, new Vertex[] { 1, 2 }, new Vertex[] { 2, 3 } });
        }

        [TestMethod]
        public void TestOrder4_Size3_Star()
        {
            bk(new Vertex[][] { new Vertex[] { 1, 2, 3 }, new Vertex[] { 0 },
                                new Vertex[] { 0 }, new Vertex[] { 0 } },
               new Vertex[][] { new Vertex[] { 0, 1 }, new Vertex[] { 0, 2 }, new Vertex[] { 0, 3 } });
        }

        [TestMethod]
        public void TestOrder4_Size4_p()
        {
            bk(new Vertex[][] { new Vertex[] { 1 }, new Vertex[] { 0, 2, 3 },
                                new Vertex[] { 1, 3 }, new Vertex[] { 1, 2 } },
               new Vertex[][] { new Vertex[] { 0, 1 }, new Vertex[] { 1, 2, 3 } });
        }

        [TestMethod]
        public void TestOrder4_Size4_Square()
        {
            bk(new Vertex[][] { new Vertex[] { 1, 3 }, new Vertex[] { 0, 2 },
                                new Vertex[] { 1, 3 }, new Vertex[] { 0, 2 } },
               new Vertex[][] { new Vertex[] { 0, 1 }, new Vertex[] { 0, 3 },
                                new Vertex[] { 1, 2 }, new Vertex[] { 2, 3 } });
        }

        [TestMethod]
        public void TestOrder4_Size5()
        {
            bk(new Vertex[][] { new Vertex[] { 1, 2, 3 }, new Vertex[] { 0, 2 },
                                new Vertex[] { 0, 1, 3 }, new Vertex[] { 0, 2 } },
               new Vertex[][] { new Vertex[] { 0, 1, 2 }, new Vertex[] { 0, 2, 3 } });
        }

        [TestMethod]
        public void TestOrder4_Size6()
        {
            bk(new Vertex[][] { new Vertex[] { 1, 2, 3 }, new Vertex[] { 0, 2, 3 },
                                new Vertex[] { 0, 1, 3 }, new Vertex[] { 0, 1, 2 } },
               new Vertex[][] { new Vertex[] { 0, 1, 2, 3 } });
        }

        [TestMethod]
        public void TestOrder4_Size6_Penultimate()
        {
            bk(new Vertex[][] { new Vertex[] { 1, 2, 3, 4 }, new Vertex[] { 0, 2, 3, 4 },
                                new Vertex[] { 0, 1, 3, 4 }, new Vertex[] { 0, 1, 2 },
                                new Vertex[] { 0, 1, 2 } },
               new Vertex[][] { new Vertex[] { 0, 1, 2, 3 }, new Vertex[] { 0, 1, 2, 4 } });
        }

        [TestMethod]
        public void TestSample()
        {
            bk(new Vertex[][] { new Vertex[] { },
                                new Vertex[] { 2, 3, 4 },
                                new Vertex[] { 1, 3, 4, 5 },
                                new Vertex[] { 1, 2, 4, 5 },
                                new Vertex[] { 1, 2, 3 },
                                new Vertex[] { 2, 3, 6, 7 },
                                new Vertex[] { 5, 7 },
                                new Vertex[] { 5, 6 }},
               new Vertex[][] { new Vertex[] { 1, 2, 3, 4 },
                                new Vertex[] { 2, 3, 5 },
                                new Vertex[] { 5, 6, 7 } });
        }

        [TestMethod]
        public void TestBigger()
        {
            bk(new Vertex[][] { new Vertex[] { 1, 2, 3, 4, 6, 7},
                                new Vertex[] { 0, 3, 6, 7, 8, 9 },
                                new Vertex[] { 0, 3, 5, 7, 8, 9 },
                                new Vertex[] { 0, 1, 2, 4, 9 },
                                new Vertex[] { 0, 3, 6, 7, 9 },
                                new Vertex[] { 2, 6 },
                                new Vertex[] { 0, 1, 4, 5, 9 },
                                new Vertex[] { 0, 1, 2, 4, 9 },
                                new Vertex[] { 1, 2 },
                                new Vertex[] { 1, 2, 3, 4, 6, 7 }},
               new Vertex[][] { new Vertex[] { 0, 1, 3 },
                                new Vertex[] { 0, 1, 6 },
                                new Vertex[] { 0, 1, 7 },
                                new Vertex[] { 0, 2, 3 },
                                new Vertex[] { 0, 2, 7 },
                                new Vertex[] { 0, 3, 4 },
                                new Vertex[] { 0, 4, 6 },
                                new Vertex[] { 0, 4, 7 },
                                new Vertex[] { 1, 3, 9 },
                                new Vertex[] { 1, 6, 9 },
                                new Vertex[] { 1, 7, 9 },
                                new Vertex[] { 1, 8 },
                                new Vertex[] { 2, 3, 9 },
                                new Vertex[] { 2, 5 },
                                new Vertex[] { 2, 7, 9 },
                                new Vertex[] { 2, 8 },
                                new Vertex[] { 3, 4, 9 },
                                new Vertex[] { 4, 6, 9 },
                                new Vertex[] { 4, 7, 9 },
                                new Vertex[] { 5, 6 } });
        }
    }
}
