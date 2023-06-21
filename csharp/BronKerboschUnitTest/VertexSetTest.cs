using BronKerbosch;
using NUnit.Framework;
using System;
using System.Collections.Generic;

namespace BronKerboschUnitTest
{
    public class VertexSetTests
    {
        [Test]
        public void PopArbitrary1()
        {
            HashSet<Vertex> one = new() { Vertex.Nth(1) };
            var x = HashSetMgr.PopArbitrary(one).Index();
            Assert.AreEqual(x, 1);
            Assert.Zero(one.Count);
        }

        [Test]
        public void PopArbitrary2()
        {
            HashSet<Vertex> two = new() { Vertex.Nth(1), Vertex.Nth(2) };
            var x = HashSetMgr.PopArbitrary(two).Index();
            var y = HashSetMgr.PopArbitrary(two).Index();
            Assert.AreEqual(Math.Min(x, y), 1);
            Assert.AreEqual(Math.Max(x, y), 2);
            Assert.Zero(two.Count);
        }

        [Test]
        public void Overlaps()
        {
            HashSet<Vertex> empty = new();
            HashSet<Vertex> one = new() { Vertex.Nth(1) };
            HashSet<Vertex> two = new() { Vertex.Nth(1), Vertex.Nth(2) };
            HashSet<Vertex> six = new() { Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2),
                                          Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5) };
            Assert.That(!HashSetMgr.Overlaps(empty, one));
            Assert.That(!HashSetMgr.Overlaps(one, empty));
            Assert.That(!HashSetMgr.Overlaps(empty, two));
            Assert.That(!HashSetMgr.Overlaps(two, empty));
            Assert.That(!HashSetMgr.Overlaps(empty, six));
            Assert.That(!HashSetMgr.Overlaps(six, empty));
            Assert.That(HashSetMgr.Overlaps(one, two));
            Assert.That(HashSetMgr.Overlaps(two, one));
            Assert.That(HashSetMgr.Overlaps(one, six));
            Assert.That(HashSetMgr.Overlaps(six, one));
            Assert.That(HashSetMgr.Overlaps(two, six));
            Assert.That(HashSetMgr.Overlaps(six, two));
            Assert.That(HashSetMgr.Overlaps(one, one));
            Assert.That(HashSetMgr.Overlaps(two, two));
            Assert.That(HashSetMgr.Overlaps(six, six));
        }

        [Test]
        public void Intersection()
        {
            HashSet<Vertex> empty = new();
            HashSet<Vertex> one = new() { Vertex.Nth(1) };
            HashSet<Vertex> two = new() { Vertex.Nth(1), Vertex.Nth(2) };
            HashSet<Vertex> six = new() { Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2),
                                          Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5) };
            Assert.That(HashSetMgr.Intersection(empty, one).SetEquals(empty));
            Assert.That(HashSetMgr.Intersection(one, empty).SetEquals(empty));
            Assert.That(HashSetMgr.Intersection(empty, two).SetEquals(empty));
            Assert.That(HashSetMgr.Intersection(two, empty).SetEquals(empty));
            Assert.That(HashSetMgr.Intersection(empty, six).SetEquals(empty));
            Assert.That(HashSetMgr.Intersection(six, empty).SetEquals(empty));
            Assert.That(HashSetMgr.Intersection(one, two).SetEquals(one));
            Assert.That(HashSetMgr.Intersection(two, one).SetEquals(one));
            Assert.That(HashSetMgr.Intersection(one, six).SetEquals(one));
            Assert.That(HashSetMgr.Intersection(six, one).SetEquals(one));
            Assert.That(HashSetMgr.Intersection(two, six).SetEquals(two));
            Assert.That(HashSetMgr.Intersection(six, two).SetEquals(two));
            Assert.That(HashSetMgr.Intersection(one, one).SetEquals(one));
            Assert.That(HashSetMgr.Intersection(two, two).SetEquals(two));
            Assert.That(HashSetMgr.Intersection(six, six).SetEquals(six));
        }

        [Test]
        public void IntersectCount()
        {
            HashSet<Vertex> empty = new();
            HashSet<Vertex> one = new() { Vertex.Nth(1) };
            HashSet<Vertex> two = new() { Vertex.Nth(1), Vertex.Nth(2) };
            HashSet<Vertex> six = new() { Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2),
                                          Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5) };
            Assert.That(HashSetMgr.IntersectionSize(empty, one).Equals(0));
            Assert.That(HashSetMgr.IntersectionSize(one, empty).Equals(0));
            Assert.That(HashSetMgr.IntersectionSize(empty, two).Equals(0));
            Assert.That(HashSetMgr.IntersectionSize(two, empty).Equals(0));
            Assert.That(HashSetMgr.IntersectionSize(empty, six).Equals(0));
            Assert.That(HashSetMgr.IntersectionSize(six, empty).Equals(0));
            Assert.That(HashSetMgr.IntersectionSize(one, two).Equals(1));
            Assert.That(HashSetMgr.IntersectionSize(two, one).Equals(1));
            Assert.That(HashSetMgr.IntersectionSize(one, six).Equals(1));
            Assert.That(HashSetMgr.IntersectionSize(six, one).Equals(1));
            Assert.That(HashSetMgr.IntersectionSize(two, six).Equals(2));
            Assert.That(HashSetMgr.IntersectionSize(six, two).Equals(2));
            Assert.That(HashSetMgr.IntersectionSize(one, one).Equals(1));
            Assert.That(HashSetMgr.IntersectionSize(two, two).Equals(2));
            Assert.That(HashSetMgr.IntersectionSize(six, six).Equals(6));
        }

        [Test]
        public void Difference()
        {
            HashSet<Vertex> empty = new();
            HashSet<Vertex> one = new() { Vertex.Nth(1) };
            HashSet<Vertex> two = new() { Vertex.Nth(1), Vertex.Nth(2) };
            HashSet<Vertex> six = new() { Vertex.Nth(0), Vertex.Nth(1), Vertex.Nth(2),
                                          Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5) };
            Assert.That(HashSetMgr.Difference(empty, one).SetEquals(empty));
            Assert.That(HashSetMgr.Difference(empty, two).SetEquals(empty));
            Assert.That(HashSetMgr.Difference(empty, six).SetEquals(empty));
            Assert.That(HashSetMgr.Difference(one, one).SetEquals(empty));
            Assert.That(HashSetMgr.Difference(one, two).SetEquals(empty));
            Assert.That(HashSetMgr.Difference(one, six).SetEquals(empty));
            Assert.That(HashSetMgr.Difference(two, two).SetEquals(empty));
            Assert.That(HashSetMgr.Difference(two, six).SetEquals(empty));
            Assert.That(HashSetMgr.Difference(six, six).SetEquals(empty));
            Assert.That(HashSetMgr.Difference(one, empty).SetEquals(one));
            Assert.That(HashSetMgr.Difference(two, empty).SetEquals(two));
            Assert.That(HashSetMgr.Difference(six, empty).SetEquals(six));
            Assert.That(HashSetMgr.Difference(two, one).SetEquals(new HashSet<Vertex> { Vertex.Nth(2) }));
            Assert.That(HashSetMgr.Difference(six, one).SetEquals(new HashSet<Vertex> { Vertex.Nth(0), Vertex.Nth(2), Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5) }));
            Assert.That(HashSetMgr.Difference(six, two).SetEquals(new HashSet<Vertex> { Vertex.Nth(0), Vertex.Nth(3), Vertex.Nth(4), Vertex.Nth(5) }));
        }
    }
}
