using BronKerboschStudy;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;

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
        Assert.AreEqual(s.Mean, -1.0);
        Assert.IsTrue(Double.IsNaN(s.Variance));
        Assert.IsTrue(Double.IsNaN(s.Deviation));
    }


    [TestMethod]
    public void test_stats_2()
    {
        var s = new SampleStatistics();
        s.Put(-1);
        s.Put(1);
        Assert.AreEqual(s.Mean, 0.0);
        Assert.AreEqual(s.Variance, 2.0);
        Assert.AreEqual(s.Deviation, Math.Sqrt(2.0));
    }


    [TestMethod]
    public void test_stats_3()
    {
        var s = new SampleStatistics();
        s.Put(89);
        s.Put(90);
        s.Put(91);
        Assert.AreEqual(s.Mean, 90.0);
        Assert.AreEqual(s.Variance, 1.0);
        Assert.AreEqual(s.Deviation, 1.0);
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
        Assert.AreEqual(s.Mean, 5.0);
        Assert.AreEqual(s.Variance, 4.0);
        Assert.AreEqual(s.Deviation, 2.0);
    }
}
