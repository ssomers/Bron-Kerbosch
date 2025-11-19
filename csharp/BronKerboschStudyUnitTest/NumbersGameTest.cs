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
            Assert.That(NumbersGame.ParseInt("0"), Is.EqualTo(0));
            Assert.That(NumbersGame.ParseInt("123"), Is.EqualTo(123));
            Assert.That(NumbersGame.ParseInt("1k"), Is.EqualTo(1_000));
            Assert.That(NumbersGame.ParseInt("1M"), Is.EqualTo(1_000_000));
            Assert.That(NumbersGame.ParseInt("42M"), Is.EqualTo(42_000_000));
        }

        [Test]
        public void ParseNegativeInt()
        {
            Assert.That(NumbersGame.ParseInt("-1"), Is.EqualTo(-1));
            Assert.That(NumbersGame.ParseInt("-1M"), Is.EqualTo(-1_000_000));
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
