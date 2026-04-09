using NUnit.Framework;
using System.Collections.Generic;
using System.Linq;

namespace BronKerbosch.Test
{
    public class DegeneracyHashTest : DegeneracyTestTemplate<HashSet<Vertex>, HashSetMgr> { }
    public class DegeneracySortedTest : DegeneracyTestTemplate<SortedSet<Vertex>, SortedSetMgr> { }

    public class DegeneracyTestTemplate<TVertexSet, TVertexSetMgr> : LabGraphs<TVertexSet, TVertexSetMgr>
        where TVertexSet : ISet<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        [Test]
        public void TestOrder0()
        {
            var g = Order0.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g), Is.Empty);
        }

        [Test]
        public void TestOrder1()
        {
            var g = Order1.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g), Is.Empty);
        }

        [Test]
        public void TestOrder2_isolated()
        {
            var g = Order2_isolated.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g), Is.Empty);
        }

        [Test]
        public void TestOrder2_connected()
        {
            var g = Order2_connected.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(1));
        }

        [Test]
        public void TestOrder3_size1_left()
        {
            var g = Order3_size1_left.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(1));
        }

        [Test]
        public void TestOrder3_size1_long()
        {
            var g = Order3_size1_long.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(1));
        }

        [Test]
        public void TestOrder3_size1_right()
        {
            var g = Order3_size1_right.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(1));
        }

        [Test]
        public void TestOrder3_size2()
        {
            var g = Order3_size2.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(2));
        }

        [Test]
        public void TestOrder3_size3()
        {
            var g = Order3_size3.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(2));
        }

        [Test]
        public void TestOrder4_size2()
        {
            var g = Order4_size2.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(2));
        }

        [Test]
        public void TestOrder4_size3_bus()
        {
            var g = Order4_size3_bus.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(3));
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Select((pair) => pair.Item1).First(), Is.AnyOf([Vertex.Nth(0), Vertex.Nth(3)]));
        }

        [Test]
        public void TestOrder4_size3_star()
        {
            var g = Order4_size3_star.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(3));
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Select((pair) => pair.Item1).First, Is.Not.EqualTo(Vertex.Nth(0)));
        }

        [Test]
        public void TestOrder4_size4_p()
        {
            var g = Order4_size4_p.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(3));
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Select((pair) => pair.Item1).First, Is.EqualTo(Vertex.Nth(0)));
        }

        [Test]
        public void TestOrder4_size4_square()
        {
            var g = Order4_size4_square.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(3));
        }

        [Test]
        public void TestOrder4_size5()
        {
            var g = Order4_size5.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(3));
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Select((pair) => pair.Item1).First, Is.AnyOf([Vertex.Nth(1), Vertex.Nth(3)]));
        }

        [Test]
        public void TestOrder5_size6_penultimate()
        {
            var g = Order5_size6_penultimate.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(4));
        }

        [Test]
        public void TestSample()
        {
            var g = Sample.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(6));
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Select((pair) => pair.Item1).First, Is.AnyOf([Vertex.Nth(6), Vertex.Nth(7)]));
        }

        [Test]
        public void TestBigger()
        {
            var g = Bigger.Graph;
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Count, Is.EqualTo(9));
            Assert.That(Degeneracy<TVertexSet, TVertexSetMgr>.Iter(g).Select((pair) => pair.Item1).First, Is.AnyOf([Vertex.Nth(5), Vertex.Nth(8)]));
        }
    }
}
