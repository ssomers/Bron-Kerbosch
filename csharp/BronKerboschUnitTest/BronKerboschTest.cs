﻿using BronKerbosch;
using Graph;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Collections.Generic;
using System.Linq;

namespace BronKerboschUnitTest
{
    [TestClass]
    public class BronKerboschTest
    {
        static int comparer(List<Vertex> lhs, List<Vertex> rhs)
        {
            for (var i = 0; i < lhs.Count && i < rhs.Count; i++)
            {
                var d = lhs[i] - rhs[i];
                if (d != 0)
                {
                    return d;
                }
            }
            Assert.Fail("got overlapping cliques of length {} <> length {}", lhs.Count, rhs.Count);
            return 0;
        }

        static void bk(Vertex[][] adjacencies, Vertex[][] cliques)
        {
            var graph = new UndirectedGraph(adjacencies.Select(neighbours => neighbours.ToHashSet()).ToList());
            var reporter = new SimpleReporter();
            BronKerbosch1.explore(graph, reporter);
            Assert.AreEqual(reporter.Cliques.Count, cliques.Length);
            foreach (List<Vertex> clique in reporter.Cliques)
                clique.Sort();
            reporter.Cliques.Sort(comparer);
            foreach ((List<Vertex> clique, int i) in reporter.Cliques.Select((v, i) => (v, i)))
                Assert.IsTrue(clique.SequenceEqual(cliques[i]));
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
        public void TestOrder3_Size1()
        {
            bk(new Vertex[][] { new Vertex[] { 1 }, new Vertex[] { 0 }, new Vertex[] { } },
               new Vertex[][] { new Vertex[] { 0, 1 } });
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
        public void TestOrder4_Size2_Isolated()
        {
            bk(new Vertex[][] { new Vertex[] { 1, 2 },
                                new Vertex[] { 0 },
                                new Vertex[] { 0 },
                                new Vertex[] { } },
               new Vertex[][] { new Vertex[] { 0, 1 }, new Vertex[] { 0, 2 } });
        }

        [TestMethod]
        public void TestOrder4_Size2_Connected()
        {
            bk(new Vertex[][] { new Vertex[] { 1 }, new Vertex[] { 0 },
                                new Vertex[] { 3 }, new Vertex[] { 2 } },
               new Vertex[][] { new Vertex[] { 0, 1 }, new Vertex[] { 2, 3 } });
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
    }
}