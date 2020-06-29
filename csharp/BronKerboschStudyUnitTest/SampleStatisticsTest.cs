using BronKerboschStudy;
using NUnit.Framework;
using System;

namespace BronKerboschStudyUnitTest
{
    public class SampleStatisticsTests
    {
        [Test]
        public void stats_0()
        {
            var s = new SampleStatistics();
            Assert.That(double.IsNaN(s.Mean));
            Assert.That(double.IsNaN(s.Variance));
            Assert.That(double.IsNaN(s.Deviation));
        }


        [Test]
        public void stats_1()
        {
            var s = new SampleStatistics();
            s.Put(-1);
            Assert.That(s.Mean.Equals(-1));
            Assert.That(double.IsNaN(s.Variance));
            Assert.That(double.IsNaN(s.Deviation));
        }


        [Test]
        public void stats_2()
        {
            var s = new SampleStatistics();
            s.Put(-1);
            s.Put(1);
            Assert.That(s.Mean.Equals(0));
            Assert.That(s.Variance.Equals(2));
            Assert.That(s.Deviation.Equals(Math.Sqrt(2)));
        }


        [Test]
        public void stats_3()
        {
            var s = new SampleStatistics();
            s.Put(89);
            s.Put(90);
            s.Put(91);
            Assert.That(s.Mean.Equals(90));
            Assert.That(s.Variance.Equals(1));
            Assert.That(s.Deviation.Equals(1));
        }


        [Test]
        public void stats_9()
        {
            var s = new SampleStatistics();
            s.Put(2);
            s.Put(4);
            s.Put(4);
            s.Put(4);
            s.Put(5);
            s.Put(5);
            s.Put(5);
            s.Put(7);
            s.Put(9);
            Assert.That(s.Mean.Equals(5));
            Assert.That(s.Variance.Equals(4));
            Assert.That(s.Deviation.Equals(2));
        }
    }
}
