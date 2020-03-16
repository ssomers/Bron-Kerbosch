using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using Vertex = System.UInt32;

namespace BronKerboschUnitTest
{
    [TestClass]
    public class UtilTest
    {
        [TestMethod]
        public void Util_Append()
        {
            var empty = ImmutableArray.Create<Vertex>();
            var one = Util.Append(empty, 11);
            var two = Util.Append(one, 22);
            Assert.AreEqual(0, empty.Length);
            Assert.AreEqual(1, one.Length);
            Assert.AreEqual(2, two.Length);
            Assert.AreEqual(11u, one[0]);
            Assert.AreEqual(11u, two[0]);
            Assert.AreEqual(22u, two[1]);
        }

        [TestMethod]
        public void Util_PopArbitrary()
        {
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            Assert.AreEqual(1u, Util.PopArbitrary(one));
            Assert.AreEqual(0, one.Count());
            var x = Util.PopArbitrary(two);
            var y = Util.PopArbitrary(two);
            Assert.AreEqual(1u, Math.Min(x, y));
            Assert.AreEqual(2u, Math.Max(x, y));
        }

        [TestMethod]
        public void Util_AreDisjoint()
        {
            var empty = new HashSet<Vertex> { };
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            var six = new HashSet<Vertex> { 0, 1, 2, 3, 4, 5 };
            Assert.IsTrue(Util.AreDisjoint(empty, one));
            Assert.IsTrue(Util.AreDisjoint(one, empty));
            Assert.IsTrue(Util.AreDisjoint(empty, two));
            Assert.IsTrue(Util.AreDisjoint(two, empty));
            Assert.IsTrue(Util.AreDisjoint(empty, six));
            Assert.IsTrue(Util.AreDisjoint(six, empty));
            Assert.IsFalse(Util.AreDisjoint(one, two));
            Assert.IsFalse(Util.AreDisjoint(two, one));
            Assert.IsFalse(Util.AreDisjoint(one, six));
            Assert.IsFalse(Util.AreDisjoint(six, one));
            Assert.IsFalse(Util.AreDisjoint(two, six));
            Assert.IsFalse(Util.AreDisjoint(six, two));
            Assert.IsFalse(Util.AreDisjoint(one, one));
            Assert.IsFalse(Util.AreDisjoint(two, two));
            Assert.IsFalse(Util.AreDisjoint(six, six));
        }

        [TestMethod]
        public void Util_Intersection()
        {
            var empty = new HashSet<Vertex> { };
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            var six = new HashSet<Vertex> { 0, 1, 2, 3, 4, 5 };
            Assert.IsTrue(Util.Intersection(empty, one).SetEquals(empty));
            Assert.IsTrue(Util.Intersection(one, empty).SetEquals(empty));
            Assert.IsTrue(Util.Intersection(empty, two).SetEquals(empty));
            Assert.IsTrue(Util.Intersection(two, empty).SetEquals(empty));
            Assert.IsTrue(Util.Intersection(empty, six).SetEquals(empty));
            Assert.IsTrue(Util.Intersection(six, empty).SetEquals(empty));
            Assert.IsTrue(Util.Intersection(one, two).SetEquals(one));
            Assert.IsTrue(Util.Intersection(two, one).SetEquals(one));
            Assert.IsTrue(Util.Intersection(one, six).SetEquals(one));
            Assert.IsTrue(Util.Intersection(six, one).SetEquals(one));
            Assert.IsTrue(Util.Intersection(two, six).SetEquals(two));
            Assert.IsTrue(Util.Intersection(six, two).SetEquals(two));
            Assert.IsTrue(Util.Intersection(one, one).SetEquals(one));
            Assert.IsTrue(Util.Intersection(two, two).SetEquals(two));
            Assert.IsTrue(Util.Intersection(six, six).SetEquals(six));
        }

        [TestMethod]
        public void Util_IntersectCount()
        {
            var empty = new HashSet<Vertex> { };
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            var six = new HashSet<Vertex> { 0, 1, 2, 3, 4, 5 };
            Assert.AreEqual(0, Util.IntersectionSize(empty, one));
            Assert.AreEqual(0, Util.IntersectionSize(one, empty));
            Assert.AreEqual(0, Util.IntersectionSize(empty, two));
            Assert.AreEqual(0, Util.IntersectionSize(two, empty));
            Assert.AreEqual(0, Util.IntersectionSize(empty, six));
            Assert.AreEqual(0, Util.IntersectionSize(six, empty));
            Assert.AreEqual(1, Util.IntersectionSize(one, two));
            Assert.AreEqual(1, Util.IntersectionSize(two, one));
            Assert.AreEqual(1, Util.IntersectionSize(one, six));
            Assert.AreEqual(1, Util.IntersectionSize(six, one));
            Assert.AreEqual(2, Util.IntersectionSize(two, six));
            Assert.AreEqual(2, Util.IntersectionSize(six, two));
            Assert.AreEqual(1, Util.IntersectionSize(one, one));
            Assert.AreEqual(2, Util.IntersectionSize(two, two));
            Assert.AreEqual(6, Util.IntersectionSize(six, six));
        }

        [TestMethod]
        public void Util_Difference()
        {
            var empty = new HashSet<Vertex> { };
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            var six = new HashSet<Vertex> { 0, 1, 2, 3, 4, 5 };
            Assert.IsTrue(Util.Difference(empty, one).SetEquals(empty));
            Assert.IsTrue(Util.Difference(empty, two).SetEquals(empty));
            Assert.IsTrue(Util.Difference(empty, six).SetEquals(empty));
            Assert.IsTrue(Util.Difference(one, one).SetEquals(empty));
            Assert.IsTrue(Util.Difference(one, two).SetEquals(empty));
            Assert.IsTrue(Util.Difference(one, six).SetEquals(empty));
            Assert.IsTrue(Util.Difference(two, two).SetEquals(empty));
            Assert.IsTrue(Util.Difference(two, six).SetEquals(empty));
            Assert.IsTrue(Util.Difference(six, six).SetEquals(empty));
            Assert.IsTrue(Util.Difference(one, empty).SetEquals(one));
            Assert.IsTrue(Util.Difference(two, empty).SetEquals(two));
            Assert.IsTrue(Util.Difference(six, empty).SetEquals(six));
            Assert.IsTrue(Util.Difference(two, one).SetEquals(new HashSet<Vertex> { 2 }));
            Assert.IsTrue(Util.Difference(six, one).SetEquals(new HashSet<Vertex> { 0, 2, 3, 4, 5 }));
            Assert.IsTrue(Util.Difference(six, two).SetEquals(new HashSet<Vertex> { 0, 3, 4, 5 }));
        }
    }
}
