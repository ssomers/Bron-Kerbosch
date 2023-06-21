using BronKerbosch;
using NUnit.Framework;
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
            Assert.AreEqual(empty.Length, 0);
            Assert.AreEqual(one.Length, 1);
            Assert.AreEqual(two.Length, 2);
            Assert.AreEqual(one[0], 11u);
            Assert.AreEqual(two[0], 11u);
            Assert.AreEqual(two[1], 22u);
        }
    }
}
