using BronKerbosch;
using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;

namespace BronKerboschUnitTest
{
    public class CollectionsUtilTests
    {
        [Test]
        public void Append()
        {
            var empty = ImmutableArray.Create<uint>();
            var one = CollectionsUtil.Append(empty, 11u);
            var two = CollectionsUtil.Append(one, 22u);
            Assert.That(empty.Length.Equals(0));
            Assert.That(one.Length.Equals(1));
            Assert.That(two.Length.Equals(2));
            Assert.That(one[0].Equals(11u));
            Assert.That(two[0].Equals(11u));
            Assert.That(two[1].Equals(22u));
        }

        [Test]
        public void PopArbitrary1()
        {
            HashSet<int> one = new() { 1 };
            var x = CollectionsUtil.PopArbitrary(one);
            Assert.That(x.Equals(1));
            Assert.Zero(one.Count);
        }

        [Test]
        public void PopArbitrary2()
        {
            HashSet<int> two = new() { 1, 2 };
            var x = CollectionsUtil.PopArbitrary(two);
            var y = CollectionsUtil.PopArbitrary(two);
            Assert.That(Math.Min(x, y).Equals(1));
            Assert.That(Math.Max(x, y).Equals(2));
            Assert.Zero(two.Count);
        }

        [Test]
        public void AreDisjoint()
        {
            HashSet<int> empty = new();
            HashSet<int> one = new() { 1 };
            HashSet<int> two = new() { 1, 2 };
            HashSet<int> six = new() { 0, 1, 2, 3, 4, 5 };
            Assert.That(!CollectionsUtil.Overlaps(empty, one));
            Assert.That(!CollectionsUtil.Overlaps(one, empty));
            Assert.That(!CollectionsUtil.Overlaps(empty, two));
            Assert.That(!CollectionsUtil.Overlaps(two, empty));
            Assert.That(!CollectionsUtil.Overlaps(empty, six));
            Assert.That(!CollectionsUtil.Overlaps(six, empty));
            Assert.That(CollectionsUtil.Overlaps(one, two));
            Assert.That(CollectionsUtil.Overlaps(two, one));
            Assert.That(CollectionsUtil.Overlaps(one, six));
            Assert.That(CollectionsUtil.Overlaps(six, one));
            Assert.That(CollectionsUtil.Overlaps(two, six));
            Assert.That(CollectionsUtil.Overlaps(six, two));
            Assert.That(CollectionsUtil.Overlaps(one, one));
            Assert.That(CollectionsUtil.Overlaps(two, two));
            Assert.That(CollectionsUtil.Overlaps(six, six));
        }

        [Test]
        public void Intersection()
        {
            HashSet<int> empty = new();
            HashSet<int> one = new() { 1 };
            HashSet<int> two = new() { 1, 2 };
            HashSet<int> six = new() { 0, 1, 2, 3, 4, 5 };
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
        public void IntersectCount()
        {
            HashSet<int> empty = new();
            HashSet<int> one = new() { 1 };
            HashSet<int> two = new() { 1, 2 };
            HashSet<int> six = new() { 0, 1, 2, 3, 4, 5 };
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
        public void Difference()
        {
            HashSet<int> empty = new();
            HashSet<int> one = new() { 1 };
            HashSet<int> two = new() { 1, 2 };
            HashSet<int> six = new() { 0, 1, 2, 3, 4, 5 };
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
            Assert.That(CollectionsUtil.Difference(two, one).SetEquals(new HashSet<int> { 2 }));
            Assert.That(CollectionsUtil.Difference(six, one).SetEquals(new HashSet<int> { 0, 2, 3, 4, 5 }));
            Assert.That(CollectionsUtil.Difference(six, two).SetEquals(new HashSet<int> { 0, 3, 4, 5 }));
        }
    }
}
