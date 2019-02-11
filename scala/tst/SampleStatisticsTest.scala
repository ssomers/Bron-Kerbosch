class SampleStatisticsTest extends org.scalatest.FunSuite {
  test("0") {
    val s = new SampleStatistics
    assert(s.mean().isNaN)
    assert(s.variance().isNaN)
    assert(s.deviation().isNaN)
  }

  test("1") {
    val s = new SampleStatistics
    s.put(-1)
    assert(s.mean == -1.0)
    assert(s.variance().isNaN)
    assert(s.deviation().isNaN)
  }

  test("2") {
    val s = new SampleStatistics
    s.put(-1)
    s.put(1)
    assert(s.mean() == 0.0)
    assert(s.variance() == 2.0)
    assert(s.deviation() == math.sqrt(2.0))
  }

  test("3") {
    val s = new SampleStatistics
    s.put(89)
    s.put(90)
    s.put(91)
    assert(s.mean() == 90.0)
    assert(s.variance() == 1.0)
    assert(s.deviation() == 1.0)
  }

  test("9") {
    val s = new SampleStatistics
    s.put(2)
    s.put(4)
    s.put(4)
    s.put(4)
    s.put(5)
    s.put(5)
    s.put(5)
    s.put(7)
    s.put(9)
    assert(s.mean() == 5.0)
    assert(s.variance() == 4.0)
    assert(s.deviation() == 2.0)
  }
}
