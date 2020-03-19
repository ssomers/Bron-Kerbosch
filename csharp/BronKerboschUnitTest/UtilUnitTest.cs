using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using Vertex = System.UInt32;

namespace BronKerboschUnitTest
{
    public class UtilTest
    {
        [Test]
        public void Util_Append()
        {
            var empty = ImmutableArray.Create<Vertex>();
            var one = CollectionsUtil.Append(empty, 11);
            var two = CollectionsUtil.Append(one, 22);
            Assert.That(empty.Length.Equals(0));
            Assert.That(one.Length.Equals(1));
            Assert.That(two.Length.Equals(2));
            Assert.That(one[0].Equals(11u));
            Assert.That(two[0].Equals(11u));
            Assert.That(two[1].Equals(22u));
        }

        [Test]
        public void Util_PopArbitrary()
        {
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            Assert.That(CollectionsUtil.PopArbitrary(one).Equals(1u));
            Assert.Zero(one.Count());
            var x = CollectionsUtil.PopArbitrary(two);
            var y = CollectionsUtil.PopArbitrary(two);
            Assert.That(Math.Min(x, y).Equals(1u));
            Assert.That(Math.Max(x, y).Equals(2u));
        }

        [Test]
        public void Util_AreDisjoint()
        {
            var empty = new HashSet<Vertex> { };
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            var six = new HashSet<Vertex> { 0, 1, 2, 3, 4, 5 };
            Assert.That(CollectionsUtil.AreDisjoint(empty, one));
            Assert.That(CollectionsUtil.AreDisjoint(one, empty));
            Assert.That(CollectionsUtil.AreDisjoint(empty, two));
            Assert.That(CollectionsUtil.AreDisjoint(two, empty));
            Assert.That(CollectionsUtil.AreDisjoint(empty, six));
            Assert.That(CollectionsUtil.AreDisjoint(six, empty));
            Assert.That(!CollectionsUtil.AreDisjoint(one, two));
            Assert.That(!CollectionsUtil.AreDisjoint(two, one));
            Assert.That(!CollectionsUtil.AreDisjoint(one, six));
            Assert.That(!CollectionsUtil.AreDisjoint(six, one));
            Assert.That(!CollectionsUtil.AreDisjoint(two, six));
            Assert.That(!CollectionsUtil.AreDisjoint(six, two));
            Assert.That(!CollectionsUtil.AreDisjoint(one, one));
            Assert.That(!CollectionsUtil.AreDisjoint(two, two));
            Assert.That(!CollectionsUtil.AreDisjoint(six, six));
        }

        [Test]
        public void Util_Intersection()
        {
            var empty = new HashSet<Vertex> { };
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            var six = new HashSet<Vertex> { 0, 1, 2, 3, 4, 5 };
            Assert.That(CollectionsUtil.Intersection(empty, one).SetEquals(empty));
            Assert.That(CollectionsUtil.Intersection(one, empty).SetEquals(empty));
            Assert.That(CollectionsUtil.Intersection(empty, two).SetEquals(empty));
            Assert.That(CollectionsUtil.Intersection(two, empty).SetEquals(empty));
            Assert.That(CollectionsUtil.Intersection(empty, six).SetEquals(empty));
            Assert.That(CollectionsUtil.Intersection(six, empty).SetEquals(empty));
            Assert.That(CollectionsUtil.Intersection(one, two).SetEquals(one));
            Assert.That(CollectionsUtil.Intersection(two, one).SetEquals(one));
            Assert.That(CollectionsUtil.Intersection(one, six).SetEquals(one));
            Assert.That(CollectionsUtil.Intersection(six, one).SetEquals(one));
            Assert.That(CollectionsUtil.Intersection(two, six).SetEquals(two));
            Assert.That(CollectionsUtil.Intersection(six, two).SetEquals(two));
            Assert.That(CollectionsUtil.Intersection(one, one).SetEquals(one));
            Assert.That(CollectionsUtil.Intersection(two, two).SetEquals(two));
            Assert.That(CollectionsUtil.Intersection(six, six).SetEquals(six));
        }

        [Test]
        public void Util_IntersectCount()
        {
            var empty = new HashSet<Vertex> { };
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            var six = new HashSet<Vertex> { 0, 1, 2, 3, 4, 5 };
            Assert.That(CollectionsUtil.IntersectionSize(empty, one).Equals(0));
            Assert.That(CollectionsUtil.IntersectionSize(one, empty).Equals(0));
            Assert.That(CollectionsUtil.IntersectionSize(empty, two).Equals(0));
            Assert.That(CollectionsUtil.IntersectionSize(two, empty).Equals(0));
            Assert.That(CollectionsUtil.IntersectionSize(empty, six).Equals(0));
            Assert.That(CollectionsUtil.IntersectionSize(six, empty).Equals(0));
            Assert.That(CollectionsUtil.IntersectionSize(one, two).Equals(1));
            Assert.That(CollectionsUtil.IntersectionSize(two, one).Equals(1));
            Assert.That(CollectionsUtil.IntersectionSize(one, six).Equals(1));
            Assert.That(CollectionsUtil.IntersectionSize(six, one).Equals(1));
            Assert.That(CollectionsUtil.IntersectionSize(two, six).Equals(2));
            Assert.That(CollectionsUtil.IntersectionSize(six, two).Equals(2));
            Assert.That(CollectionsUtil.IntersectionSize(one, one).Equals(1));
            Assert.That(CollectionsUtil.IntersectionSize(two, two).Equals(2));
            Assert.That(CollectionsUtil.IntersectionSize(six, six).Equals(6));
        }

        [Test]
        public void Util_Difference()
        {
            var empty = new HashSet<Vertex> { };
            var one = new HashSet<Vertex> { 1 };
            var two = new HashSet<Vertex> { 1, 2 };
            var six = new HashSet<Vertex> { 0, 1, 2, 3, 4, 5 };
            Assert.That(CollectionsUtil.Difference(empty, one).SetEquals(empty));
            Assert.That(CollectionsUtil.Difference(empty, two).SetEquals(empty));
            Assert.That(CollectionsUtil.Difference(empty, six).SetEquals(empty));
            Assert.That(CollectionsUtil.Difference(one, one).SetEquals(empty));
            Assert.That(CollectionsUtil.Difference(one, two).SetEquals(empty));
            Assert.That(CollectionsUtil.Difference(one, six).SetEquals(empty));
            Assert.That(CollectionsUtil.Difference(two, two).SetEquals(empty));
            Assert.That(CollectionsUtil.Difference(two, six).SetEquals(empty));
            Assert.That(CollectionsUtil.Difference(six, six).SetEquals(empty));
            Assert.That(CollectionsUtil.Difference(one, empty).SetEquals(one));
            Assert.That(CollectionsUtil.Difference(two, empty).SetEquals(two));
            Assert.That(CollectionsUtil.Difference(six, empty).SetEquals(six));
            Assert.That(CollectionsUtil.Difference(two, one).SetEquals(new HashSet<Vertex> { 2 }));
            Assert.That(CollectionsUtil.Difference(six, one).SetEquals(new HashSet<Vertex> { 0, 2, 3, 4, 5 }));
            Assert.That(CollectionsUtil.Difference(six, two).SetEquals(new HashSet<Vertex> { 0, 3, 4, 5 }));
        }
    }
}
