using BronKerboschStudy;
using NUnit.Framework;
using System;
#pragma warning disable IDE0022 // Use expression body for methods
#pragma warning disable IDE0058 // Expression value is never used

namespace BronKerboschStudyUnitTest
{
    public class RandomGraphTests
    {
        [Test]
        public void ParsePositiveInt()
        {
            Assert.AreEqual(RandomUndirectedGraph.ParseInt("0"), 0);
            Assert.AreEqual(RandomUndirectedGraph.ParseInt("123"), 123);
            Assert.AreEqual(RandomUndirectedGraph.ParseInt("1k"), 1_000);
            Assert.AreEqual(RandomUndirectedGraph.ParseInt("1M"), 1_000_000);
            Assert.AreEqual(RandomUndirectedGraph.ParseInt("42M"), 42_000_000);
        }

        [Test]
        public void ParseNegativeInt()
        {
            Assert.AreEqual(RandomUndirectedGraph.ParseInt("-1"), -1);
        }

        [Test]
        public void ParseEmpty()
        {
            Assert.Catch<FormatException>(() => RandomUndirectedGraph.ParseInt(""));
        }

        [Test]
        public void ParseUnknownSuffix()
        {
            Assert.Catch<FormatException>(() => RandomUndirectedGraph.ParseInt("1K"));
        }

        [Test]
        public void ParseNonInt()
        {
            Assert.Catch<FormatException>(() => RandomUndirectedGraph.ParseInt("1.1"));
        }
    }
}
