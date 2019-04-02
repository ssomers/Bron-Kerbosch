class SampleStatistics {
  var max: Long = 0
  var min: Long = 0
  var samples: Int = 0
  var sum: Double = 0
  var sum_of_squares: Double = 0

  def put(v: Long): Unit = {
    if (samples == 0) {
      min = v
      max = v
    } else if (min > v) {
      min = v
    } else if (max < v) {
      max = v
    }
    samples += 1
    sum += v
    sum_of_squares += v * v
  }

  def mean(): Double = {
    if (samples > 0) {
      sum / samples
    } else {
      Double.NaN
    }
  }
  def variance(): Double = {
    if (samples > 1) {
      val n = samples
      math.max(sum_of_squares - sum * sum / n, 0) / (n - 1.0)
    } else {
      Double.NaN
    }
  }
  def deviation(): Double = {
    math.sqrt(variance())
  }
}
