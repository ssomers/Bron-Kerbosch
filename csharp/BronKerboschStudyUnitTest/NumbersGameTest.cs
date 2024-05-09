using BronKerboschStudy;
using NUnit.Framework;
using System;
#pragma warning disable IDE0022 // Use expression body for method
#pragma warning disable IDE0058 // Expression value is never used

namespace BronKerboschStudyUnitTest
{
    public class NumbersGameTests
    {
        [Test]
        public void ParsePositiveInt()
        {
            Assert.AreEqual(NumbersGame.ParseInt("0"), 0);
            Assert.AreEqual(NumbersGame.ParseInt("123"), 123);
            Assert.AreEqual(NumbersGame.ParseInt("1k"), 1_000);
            Assert.AreEqual(NumbersGame.ParseInt("1M"), 1_000_000);
            Assert.AreEqual(NumbersGame.ParseInt("42M"), 42_000_000);
        }

        [Test]
        public void ParseNegativeInt()
        {
            Assert.AreEqual(NumbersGame.ParseInt("-1"), -1);
        }

        [Test]
        public void ParseEmpty()
        {
            Assert.Catch<FormatException>(() => NumbersGame.ParseInt(""));
        }

        [Test]
        public void ParseUnknownSuffix()
        {
            Assert.Catch<FormatException>(() => NumbersGame.ParseInt("1K"));
        }

        [Test]
        public void ParseNonInt()
        {
            Assert.Catch<FormatException>(() => NumbersGame.ParseInt("1.1"));
        }
    }
}
