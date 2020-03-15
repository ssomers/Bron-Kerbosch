using BronKerboschStudy;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;

namespace BronKerboschStudy
{
    [TestClass]
    public class SampleStatisticsTest
    {
        [TestMethod]
        public void test_stats_0()
        {
            var s = new SampleStatistics();
            Assert.IsTrue(Double.IsNaN(s.Mean));
            Assert.IsTrue(Double.IsNaN(s.Variance));
            Assert.IsTrue(Double.IsNaN(s.Deviation));
        }


        [TestMethod]
        public void test_stats_1()
        {
            var s = new SampleStatistics();
            s.Put(-1);
            Assert.AreEqual(-1, s.Mean);
            Assert.IsTrue(Double.IsNaN(s.Variance));
            Assert.IsTrue(Double.IsNaN(s.Deviation));
        }


        [TestMethod]
        public void test_stats_2()
        {
            var s = new SampleStatistics();
            s.Put(-1);
            s.Put(1);
            Assert.AreEqual(0, s.Mean);
            Assert.AreEqual(2, s.Variance);
            Assert.AreEqual(Math.Sqrt(2), s.Deviation);
        }


        [TestMethod]
        public void test_stats_3()
        {
            var s = new SampleStatistics();
            s.Put(89);
            s.Put(90);
            s.Put(91);
            Assert.AreEqual(90, s.Mean);
            Assert.AreEqual(1, s.Variance);
            Assert.AreEqual(1, s.Deviation);
        }


        [TestMethod]
        public void test_stats_9()
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
            Assert.AreEqual(5, s.Mean);
            Assert.AreEqual(4, s.Variance);
            Assert.AreEqual(2, s.Deviation);
        }
    }
}
