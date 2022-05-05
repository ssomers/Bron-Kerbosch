package Stats

import (
	"BronKerboschStudy/Assert"
	"math"
	"testing"
)

func TestStats0(t *testing.T) {
	var s SampleStatistics
	Assert.IsTrue(math.IsNaN(s.Mean()))
	Assert.IsTrue(math.IsNaN(s.Variance()))
	Assert.IsTrue(math.IsNaN(s.Deviation()))
}

func TestStats1(t *testing.T) {
	var s SampleStatistics
	s.Put(-1)
	Assert.AreEqual(s.Mean(), -1.0)
	Assert.IsTrue(math.IsNaN(s.Variance()))
	Assert.IsTrue(math.IsNaN(s.Deviation()))
}

func TestStats2(t *testing.T) {
	var s SampleStatistics
	s.Put(1.0)
	s.Put(2.0)
	Assert.AreEqual(s.Mean(), 1.5)
	Assert.AreEqual(s.Variance(), 0.5)
	Assert.AreEqual(s.Deviation(), math.Sqrt(0.5))
}

func TestStats3(t *testing.T) {
	var s SampleStatistics
	s.Put(89)
	s.Put(90)
	s.Put(91)
	Assert.AreEqual(s.Mean(), 90.0)
	Assert.AreEqual(s.Variance(), 1.0)
	Assert.AreEqual(s.Deviation(), 1.0)
}

func TestStats9(t *testing.T) {
	var s SampleStatistics
	s.Put(2)
	s.Put(4)
	s.Put(4)
	s.Put(4)
	s.Put(5)
	s.Put(5)
	s.Put(5)
	s.Put(7)
	s.Put(9)
	Assert.AreEqual(s.Mean(), 5.0)
	Assert.AreEqual(s.Variance(), 4.0)
	Assert.AreEqual(s.Deviation(), 2.0)
}

func FuzzStats1(f *testing.F) {
	f.Add(1.)
	f.Fuzz(func(t *testing.T, x float64) {
		var s SampleStatistics
		s.Put(x)
		Assert.IsTrue(s.Mean() == x)
	})
}

func FuzzStats2(f *testing.F) {
	f.Add(1., 2.)
	f.Fuzz(func(t *testing.T, x float64, y float64) {
		var s SampleStatistics
		s.Put(x)
		s.Put(y)
		Assert.IsTrue(s.Mean() >= s.Min())
		Assert.IsTrue(s.Mean() <= s.Max())
		Assert.IsTrue(s.Variance() >= 0.)
		Assert.IsTrue(s.Deviation() <= (s.Max()-s.Min())*1.5)
	})
}

func FuzzStatsN(f *testing.F) {
	f.Add(1., uint16(3))
	f.Fuzz(func(t *testing.T, x float64, n uint16) {
		var s SampleStatistics
		for i := n; i > 0; i-- {
			s.Put(x)
		}
		Assert.IsTrue(s.Mean() >= s.Min())
		Assert.IsTrue(s.Mean() <= s.Max())
		Assert.IsTrue(s.Variance() >= 0.)
		Assert.IsTrue(s.Deviation() <= (s.Max()-s.Min())*1.5)
	})
}
