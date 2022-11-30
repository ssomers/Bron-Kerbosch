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
            Assert.That(empty.Length.Equals(0));
            Assert.That(one.Length.Equals(1));
            Assert.That(two.Length.Equals(2));
            Assert.That(one[0].Equals(11u));
            Assert.That(two[0].Equals(11u));
            Assert.That(two[1].Equals(22u));
        }
    }
}
