using NUnit.Framework;
using System.Collections.Generic;

namespace BronKerbosch.Test
{
    public class UndirectedGraphHashTest : UndirectedGraphTestTemplate<HashSet<Vertex>, HashSetMgr> { }
    public class UndirectedGraphSortedTest : UndirectedGraphTestTemplate<SortedSet<Vertex>, SortedSetMgr> { }

    public class UndirectedGraphTestTemplate<TVertexSet, TVertexSetMgr> : LabGraphs<TVertexSet, TVertexSetMgr>
        where TVertexSet : ISet<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        [Test]
        public void TestOrder0()
        {
            var g = Order0.Graph;
            Assert.That(g.Order, Is.EqualTo(0));
            Assert.That(g.Size, Is.EqualTo(0));
            Assert.That(g.MaxDegree, Is.EqualTo(0));
            Assert.That(g.Vertices(), Is.Empty);
            Assert.That(g.ConnectedVertices(), Is.Empty);
            Assert.That(g.MaxDegreeVertices(), Is.Empty);
        }

        [Test]
        public void TestOrder1()
        {
            var g = Order1.Graph;
            Assert.That(g.Order, Is.EqualTo(1));
            Assert.That(g.Size, Is.EqualTo(0));
            Assert.That(g.MaxDegree, Is.EqualTo(0));
            Assert.That(g.Vertices(), Is.EqualTo([Vertex.Nth(0)]));
            Assert.That(g.ConnectedVertices(), Is.Empty);
            Assert.That(g.MaxDegreeVertices(), Is.EqualTo([Vertex.Nth(0)]));
        }

        [Test]
        public void TestOrder2_isolated()
        {
            var g = Order2_isolated.Graph;
            Assert.That(g.Order, Is.EqualTo(2));
            Assert.That(g.Size, Is.EqualTo(0));
            Assert.That(g.MaxDegree, Is.EqualTo(0));

            Assert.That(g.Vertices(), Is.EqualTo([Vertex.Nth(0), Vertex.Nth(1)]));

            Assert.That(g.ConnectedVertices(), Is.Empty);
            Assert.That(g.MaxDegreeVertices(), Is.EqualTo(g.Vertices()));
        }

        [Test]
        public void TestOrder2_connected()
        {
            var g = Order2_connected.Graph;
            Assert.That(g.Order, Is.EqualTo(2));
            Assert.That(g.Size, Is.EqualTo(1));
            Assert.That(g.MaxDegree, Is.EqualTo(1));

            Assert.That(g.Vertices(), Is.EqualTo([Vertex.Nth(0), Vertex.Nth(1)]));

            Assert.That(g.ConnectedVertices(), Is.EqualTo(g.Vertices()));
            Assert.That(g.MaxDegreeVertices(), Is.EqualTo(g.Vertices()));
        }

        [Test]
        public void TestOrder3_size1_left()
        {
            var g = Order3_size1_left.Graph;
            Assert.That(g.Order, Is.EqualTo(3));
            Assert.That(g.Size, Is.EqualTo(1));
            Assert.That(g.MaxDegree, Is.EqualTo(1));

            Assert.That(g.Vertices(), Is.EqualTo([Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2)]));

            Assert.That(g.ConnectedVertices(), Is.EqualTo([Vertex.Nth(0), Vertex.Nth(1)]));

            Assert.That(g.MaxDegreeVertices(), Is.EqualTo(g.ConnectedVertices()));
        }

        [Test]
        public void TestOrder3_size1_long()
        {
            var g = Order3_size1_long.Graph;
            Assert.That(g.Order, Is.EqualTo(3));
            Assert.That(g.Size, Is.EqualTo(1));
            Assert.That(g.MaxDegree, Is.EqualTo(1));

            Assert.That(g.Vertices(), Is.EqualTo([Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2)]));

            Assert.That(g.ConnectedVertices(), Is.EqualTo([Vertex.Nth(0), Vertex.Nth(2)]));

            Assert.That(g.MaxDegreeVertices(), Is.EqualTo(g.ConnectedVertices()));
        }

        [Test]
        public void TestOrder3_size1_right()
        {
            var g = Order3_size1_right.Graph;
            Assert.That(g.Order, Is.EqualTo(3));
            Assert.That(g.Size, Is.EqualTo(1));
            Assert.That(g.MaxDegree, Is.EqualTo(1));

            Assert.That(g.Vertices(), Is.EqualTo([Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2)]));

            Assert.That(g.ConnectedVertices(), Is.EqualTo([Vertex.Nth(1), Vertex.Nth(2)]));

            Assert.That(g.MaxDegreeVertices(), Is.EqualTo(g.ConnectedVertices()));
        }

        [Test]
        public void TestOrder3_size2()
        {
            var g = Order3_size2.Graph;
            Assert.That(g.Order, Is.EqualTo(3));
            Assert.That(g.Size, Is.EqualTo(2));
            Assert.That(g.MaxDegree, Is.EqualTo(2));

            Assert.That(g.Vertices(), Is.EqualTo([Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2)]));

            Assert.That(g.ConnectedVertices(), Is.EqualTo(g.Vertices()));
            Assert.That(g.MaxDegreeVertices(), Is.EqualTo([Vertex.Nth(1)]));
        }

        [Test]
        public void TestOrder3_size3()
        {
            var g = Order3_size3.Graph;
            Assert.That(g.Order, Is.EqualTo(3));
            Assert.That(g.Size, Is.EqualTo(3));
            Assert.That(g.MaxDegree, Is.EqualTo(2));

            Assert.That(g.Vertices(), Is.EqualTo([Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2)]));

            Assert.That(g.ConnectedVertices(), Is.EqualTo(g.Vertices()));
            Assert.That(g.MaxDegreeVertices(), Is.EqualTo(g.Vertices()));
        }
    }
}
