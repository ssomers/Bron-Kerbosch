using NUnit.Framework;
using System;
using System.Collections.Generic;

namespace BronKerbosch.Test
{
    public class VertexSetHashTest : VertexSetTestTemplate<HashSet<Vertex>, HashSetMgr> { }
    public class VertexSetSortedTest : VertexSetTestTemplate<SortedSet<Vertex>, SortedSetMgr> { }

    public class VertexSetTestTemplate<TVertexSet, TVertexSetMgr>
        where TVertexSet : ISet<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        [Test]
        public void PopArbitrary1()
        {
            TVertexSet one = TVertexSetMgr.From([Vertex.Nth(1)]);
            var x = one.PopArbitrary().Index();
            Assert.That(x, Is.EqualTo(1));
            Assert.That(one, Is.Empty);
        }

        [Test]
        public void PopArbitrary2()
        {
            TVertexSet two = TVertexSetMgr.From([Vertex.Nth(1), Vertex.Nth(2)]);
            var x = two.PopArbitrary().Index();
            var y = two.PopArbitrary().Index();
            Assert.That(Math.Min(x, y), Is.EqualTo(1));
            Assert.That(Math.Max(x, y), Is.EqualTo(2));
            Assert.That(two, Is.Empty);
        }

        [Test]
        public void Overlaps()
        {
            var empty = TVertexSetMgr.Empty();
            var one = TVertexSetMgr.From([Vertex.Nth(1)]);
            var two = TVertexSetMgr.From([Vertex.Nth(1), Vertex.Nth(2)]);
            var six = TVertexSetMgr.From([Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2),
                                          Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5)]);
            Assert.That(!TVertexSetMgr.Overlaps(empty, one));
            Assert.That(!TVertexSetMgr.Overlaps(one, empty));
            Assert.That(!TVertexSetMgr.Overlaps(empty, two));
            Assert.That(!TVertexSetMgr.Overlaps(two, empty));
            Assert.That(!TVertexSetMgr.Overlaps(empty, six));
            Assert.That(!TVertexSetMgr.Overlaps(six, empty));
            Assert.That(TVertexSetMgr.Overlaps(one, two));
            Assert.That(TVertexSetMgr.Overlaps(two, one));
            Assert.That(TVertexSetMgr.Overlaps(one, six));
            Assert.That(TVertexSetMgr.Overlaps(six, one));
            Assert.That(TVertexSetMgr.Overlaps(two, six));
            Assert.That(TVertexSetMgr.Overlaps(six, two));
            Assert.That(TVertexSetMgr.Overlaps(one, one));
            Assert.That(TVertexSetMgr.Overlaps(two, two));
            Assert.That(TVertexSetMgr.Overlaps(six, six));
        }

        [Test]
        public void Intersection()
        {
            var empty = TVertexSetMgr.Empty();
            var one = TVertexSetMgr.From([Vertex.Nth(1)]);
            var two = TVertexSetMgr.From([Vertex.Nth(1), Vertex.Nth(2)]);
            var six = TVertexSetMgr.From([Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2),
                                          Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5)]);
            Assert.That(TVertexSetMgr.Intersection(empty, one).SetEquals(empty));
            Assert.That(TVertexSetMgr.Intersection(one, empty).SetEquals(empty));
            Assert.That(TVertexSetMgr.Intersection(empty, two).SetEquals(empty));
            Assert.That(TVertexSetMgr.Intersection(two, empty).SetEquals(empty));
            Assert.That(TVertexSetMgr.Intersection(empty, six).SetEquals(empty));
            Assert.That(TVertexSetMgr.Intersection(six, empty).SetEquals(empty));
            Assert.That(TVertexSetMgr.Intersection(one, two).SetEquals(one));
            Assert.That(TVertexSetMgr.Intersection(two, one).SetEquals(one));
            Assert.That(TVertexSetMgr.Intersection(one, six).SetEquals(one));
            Assert.That(TVertexSetMgr.Intersection(six, one).SetEquals(one));
            Assert.That(TVertexSetMgr.Intersection(two, six).SetEquals(two));
            Assert.That(TVertexSetMgr.Intersection(six, two).SetEquals(two));
            Assert.That(TVertexSetMgr.Intersection(one, one).SetEquals(one));
            Assert.That(TVertexSetMgr.Intersection(two, two).SetEquals(two));
            Assert.That(TVertexSetMgr.Intersection(six, six).SetEquals(six));
        }

        [Test]
        public void IntersectCount()
        {
            var empty = TVertexSetMgr.Empty();
            var one = TVertexSetMgr.From([Vertex.Nth(1)]);
            var two = TVertexSetMgr.From([Vertex.Nth(1), Vertex.Nth(2)]);
            var six = TVertexSetMgr.From([Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2),
                                          Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5)]);
            Assert.That(TVertexSetMgr.IntersectionSize(empty, one).Equals(0));
            Assert.That(TVertexSetMgr.IntersectionSize(one, empty).Equals(0));
            Assert.That(TVertexSetMgr.IntersectionSize(empty, two).Equals(0));
            Assert.That(TVertexSetMgr.IntersectionSize(two, empty).Equals(0));
            Assert.That(TVertexSetMgr.IntersectionSize(empty, six).Equals(0));
            Assert.That(TVertexSetMgr.IntersectionSize(six, empty).Equals(0));
            Assert.That(TVertexSetMgr.IntersectionSize(one, two).Equals(1));
            Assert.That(TVertexSetMgr.IntersectionSize(two, one).Equals(1));
            Assert.That(TVertexSetMgr.IntersectionSize(one, six).Equals(1));
            Assert.That(TVertexSetMgr.IntersectionSize(six, one).Equals(1));
            Assert.That(TVertexSetMgr.IntersectionSize(two, six).Equals(2));
            Assert.That(TVertexSetMgr.IntersectionSize(six, two).Equals(2));
            Assert.That(TVertexSetMgr.IntersectionSize(one, one).Equals(1));
            Assert.That(TVertexSetMgr.IntersectionSize(two, two).Equals(2));
            Assert.That(TVertexSetMgr.IntersectionSize(six, six).Equals(6));
        }

        [Test]
        public void Difference()
        {
            var empty = TVertexSetMgr.Empty();
            var one = TVertexSetMgr.From([Vertex.Nth(1)]);
            var two = TVertexSetMgr.From([Vertex.Nth(1), Vertex.Nth(2)]);
            var six = TVertexSetMgr.From([Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2),
                                          Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5)]);
            Assert.That(TVertexSetMgr.Difference(empty, one).SetEquals(empty));
            Assert.That(TVertexSetMgr.Difference(empty, two).SetEquals(empty));
            Assert.That(TVertexSetMgr.Difference(empty, six).SetEquals(empty));
            Assert.That(TVertexSetMgr.Difference(one, one).SetEquals(empty));
            Assert.That(TVertexSetMgr.Difference(one, two).SetEquals(empty));
            Assert.That(TVertexSetMgr.Difference(one, six).SetEquals(empty));
            Assert.That(TVertexSetMgr.Difference(two, two).SetEquals(empty));
            Assert.That(TVertexSetMgr.Difference(two, six).SetEquals(empty));
            Assert.That(TVertexSetMgr.Difference(six, six).SetEquals(empty));
            Assert.That(TVertexSetMgr.Difference(one, empty).SetEquals(one));
            Assert.That(TVertexSetMgr.Difference(two, empty).SetEquals(two));
            Assert.That(TVertexSetMgr.Difference(six, empty).SetEquals(six));
            Assert.That(TVertexSetMgr.Difference(two, one).SetEquals(TVertexSetMgr.From([Vertex.Nth(2)])));
            Assert.That(TVertexSetMgr.Difference(six, one).SetEquals(TVertexSetMgr.From([Vertex.Nth(0), Vertex.Nth(2), Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5)])));
            Assert.That(TVertexSetMgr.Difference(six, two).SetEquals(TVertexSetMgr.From([Vertex.Nth(0), Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5)])));
        }
    }
}
